// worker.js (module)
import * as wasm from 'ib_pcode_compiler';

let controlSab = null;
let control = null;
let respBuf = null; // Uint8Array view
let reqId = 0;

self.onmessage = async (ev) => {
    const msg = ev.data;
    
    if (msg.type === 'init') {
        controlSab = msg.controlSab;
        control = new Int32Array(controlSab, 0, 1);
        respBuf = new Uint8Array(controlSab, Int32Array.BYTES_PER_ELEMENT);

        try {
            console.log("[worker] Initializing wasm init()...");
            console.log("[worker] wasm initialized");
        } catch (e) {
            console.error("[worker] wasm init() failed:", e);
        }
    } else if (msg.type === 'run') {
        try {
            console.log("[worker] ▶ Running wasm program...");
            wasm.run_program_wasm(msg.source);
            self.postMessage({ type: 'finish', text: "Program finished successfully" });
            console.log("[worker] Program finished");
        } catch (e) {
            console.error("[worker] Error during run:", e);
            if (e && e.stack) console.error(e.stack);
        }
    }
};

// This is synchronous from the worker's point of view:
// 1) set control to "waiting" (1)
// 2) postMessage to main to show UI
// 3) Atomics.wait(control, 0, 1)  < main will write response into respBuf and Atomics.notify(...)
globalThis.blocking_request_input = function (prompt) {
    const id = ++reqId;
    Atomics.store(control, 0, 1); // 1 = waiting
    console.log(`[worker] Requesting input: "${prompt}" (id=${id})`);
    self.postMessage({ type: 'request-input', id, prompt });

    // block until main writes response and sets control to 2
    Atomics.wait(control, 0, 1);

    // When woken, control[0] should be 2 (ready), otherwise - program canceled
    const state = Atomics.load(control, 0);
    if (state !== 2) {
        console.log(`[worker] Woken but state=${state}`);
        return;
    }

    // read response bytes and decode - copy out of the shared buffer first
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

    console.log(`[worker] Received input: "${res}"`);
    return res;
};

globalThis.write_output = function (s) {
    self.postMessage({ type: 'output', text: s });
};
