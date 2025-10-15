// worker.js (module)
import init, * as wasm from './pkg/ib_pseudocompiler.js';

let controlSab = null;
let control = null;
let respBuf = null; // Uint8Array view
let pendingRequest = null;
let reqId = 0;

self.onmessage = async (ev) => {
    const msg = ev.data;

    if (msg.type === 'init') {
        controlSab = msg.controlSab;
        control = new Int32Array(controlSab, 0, 1);
        // response buffer lives after the Int32 slot
        respBuf = new Uint8Array(controlSab, Int32Array.BYTES_PER_ELEMENT);

        try {
            // Try wasm-bindgen init with URL first
            try {
                console.log("[worker] Initializing wasm (init(url))...");
                await init();
                console.log("[worker] ✅ wasm initialized with URL");
                console.log("[worker] Exports:", Object.keys(wasm));
            } catch (e) {
                console.error("[worker] ❌ init(url) failed:", e);
            }

        } catch (outer) {
            console.error("[worker] ❌ Fatal init error:", outer);
            return;
        }
    } else if (msg.type === 'run') {
        try {
            if (typeof wasm.run_program_wasm !== "function") {
                console.error("[worker] ❌ wasm.run_program_wasm not found! Exports:", Object.keys(wasm));
                return;
            }
            console.log("[worker] ▶ Running wasm program...");
            wasm.run_program_wasm(msg.source);
            console.log("[worker] ✅ Program finished");
        } catch (e) {
            console.error("[worker] ❌ Error during run:", e);
            if (e && e.stack) console.error(e.stack);
        }
    }
};

// ---- Runtime hooks for Rust side ----
// This is synchronous from the worker's point of view:
// 1) set control to "waiting" (1)
// 2) postMessage to main to show UI
// 3) Atomics.wait(control, 0, 1)  <-- main will write response into respBuf and Atomics.notify(...)
globalThis.blocking_request_input = function (prompt) {
    const id = ++reqId;
    Atomics.store(control, 0, 1); // 1 = waiting
    console.log(`[worker] ⌨️  Requesting input: "${prompt}" (id=${id})`);
    self.postMessage({ type: 'request-input', id, prompt });

    // block until main writes response and sets control to 2
    Atomics.wait(control, 0, 1);

    // When woken, control[0] should be 2 (ready) — allow for spurious wakeups
    const state = Atomics.load(control, 0);
    if (state !== 2) {
        console.warn(`[worker] Woken but state=${state}`);
    }

    // read response bytes and decode — copy out of the shared buffer first
    let len = 0;
    while (len < respBuf.length && respBuf[len] !== 0) len++;
    const sharedSlice = respBuf.subarray(0, len);

    // make a non-shared copy (TextDecoder rejects shared views)
    const copy = sharedSlice.slice(); // slice() returns a new Uint8Array with copied bytes
    const dec = new TextDecoder();
    const res = dec.decode(copy);

    // clear respBuf and reset state to idle
    respBuf.fill(0);
    Atomics.store(control, 0, 0); // idle

    console.log(`[worker] 📥 Received input: "${res}"`);
    return res;
};

globalThis.write_output = function (s) {
    console.log(`[worker] 🖨️ Output: ${s}`);
    self.postMessage({ type: 'output', text: s });
};
