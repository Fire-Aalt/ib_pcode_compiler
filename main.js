// main.js
if ("serviceWorker" in navigator) {
    // Register service worker
    navigator.serviceWorker.register(new URL("./sw.js", import.meta.url)).then(
        function (registration) {
            console.log("COOP/COEP Service Worker registered", registration.scope);
            // If the registration is active, but it's not controlling the page
            if (registration.active && !navigator.serviceWorker.controller) {
                window.location.reload();
            }
        },
        function (err) {
            console.log("COOP/COEP Service Worker failed to register", err);
        }
    );
} else {
    console.warn("Cannot register a service worker");
}

// create a SharedArrayBuffer: 1 Int32 control slot + 4096 bytes for response text
const RESPONSE_BYTES = 4096;
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1); // control[0] state
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);

control[0] = 0; // 0 = idle, 1 = waiting (worker), 2 = ready (main wrote response)

const worker = new Worker('./worker.js', { type: 'module' });

const outEl = document.getElementById('out');
const promptArea = document.getElementById('prompt-area');
const promptText = document.getElementById('prompt-text');
const promptInput = document.getElementById('prompt-input');
const promptSubmit = document.getElementById('prompt-submit');

let lastRequestId = null;

worker.onmessage = (ev) => {
    const msg = ev.data;
    if (msg.type === 'wasm-ready') {
        console.log('worker wasm ready');
    } else if (msg.type === 'request-input') {
        // show UI
        lastRequestId = msg.id;
        promptText.textContent = msg.prompt;
        promptArea.style.display = 'block';
        promptInput.focus();
    } else if (msg.type === 'output') {
        outEl.innerHTML += msg.text + '\n';
    } else if (msg.type === 'error') {
        outEl.textContent += 'ERROR: ' + msg.message + '\n';
    } else {
        outEl.textContent += msg.text + '\n';
    }
};

// send the SharedArrayBuffer in an init message
worker.postMessage({ type: 'init', controlSab: sab });

promptSubmit.addEventListener('click', () => {
    if (lastRequestId == null) return;
    const text = promptInput.value || '';

    // encode into respBuf (truncate if longer than RESPONSE_BYTES-1)
    const enc = new TextEncoder();
    const encoded = enc.encode(text);
    const writeLen = Math.min(encoded.length, RESPONSE_BYTES - 1);
    respBuf.fill(0); // clear
    respBuf.set(encoded.subarray(0, writeLen));

    // signal worker that response is ready
    Atomics.store(control, 0, 2); // 2 = ready
    Atomics.notify(control, 0, 1);
    console.log(`[main] Wrote response (len ${writeLen})`);
    
    // hide UI
    lastRequestId = null;
    promptInput.value = '';
    promptArea.style.display = 'none';
});

const runBtn = document.getElementById('run');
runBtn.addEventListener('click', () => {
    Atomics.store(control, 0, 0); // 0 = idle - reset
    Atomics.notify(control, 0, 1);
    
    const src = document.getElementById('editor').value;
    outEl.textContent = '';
    worker.postMessage({ type: 'run', source: src });
});