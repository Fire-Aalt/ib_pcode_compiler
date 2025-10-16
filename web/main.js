/* main.js — module */
if ("serviceWorker" in navigator) {
    navigator.serviceWorker.register(new URL("./sw.js", import.meta.url)).then(
        (registration) => {
            console.log("COOP/COEP Service Worker registered", registration.scope);
            if (registration.active && !navigator.serviceWorker.controller) {
                window.location.reload();
            }
        },
        (err) => {
            console.log("COOP/COEP Service Worker failed to register", err);
        }
    );
} else {
    console.warn("Cannot register a service worker");
}

/* SharedArrayBuffer layout: first Int32 (control slot) then RESPONSE_BYTES bytes for response */
const RESPONSE_BYTES = 8192; // increase if you expect longer inputs
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1);
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);
control[0] = 0; // 0 = idle, 1 = waiting, 2 = ready

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

const terminal = document.getElementById('terminal');
const editor = document.getElementById('editor');
const gutter = document.getElementById('gutter');
const sampleSelect = document.getElementById('sampleSelect');
const lineNumberEl = document.getElementById('lineNumber');

const modal = document.getElementById('modal');
const modalPrompt = document.getElementById('modalPrompt');
const modalInput = document.getElementById('modalInput');
const modalOk = document.getElementById('modalOk');
const modalCancel = document.getElementById('modalCancel');

let lastRequestId = null;
let currentRunWindow = null; // popup window when using Run In New Window

// sample programs for convenience
const samples = {
    welcome: `output "Welcome"\nloop COUNT from 1 to 5\n  output COUNT\nend loop`,
    input_demo: `output "Enter something"\ninput "Your value:"\noutput "Done"`,
};
for (const k of Object.keys(samples)) {
    const opt = document.createElement('option');
    opt.value = k; opt.textContent = k;
    sampleSelect.appendChild(opt);
}
sampleSelect.addEventListener('change', () => {
    const v = sampleSelect.value; if (!v) return;
    editor.value = samples[v]; updateGutter(); updateLineNumber();
});

// gutter and line sync
function updateGutter() {
    const lines = Math.max(1, editor.value.split('\n').length);
    gutter.innerHTML = '';
    for (let i = 1; i <= lines; i++) {
        const d = document.createElement('div'); d.textContent = i; gutter.appendChild(d);
    }
}
editor.addEventListener('input', () => { updateGutter(); updateLineNumber(); });
editor.addEventListener('scroll', () => { gutter.scrollTop = editor.scrollTop; terminal.scrollTop = terminal.scrollHeight; });
editor.addEventListener('keydown', (e) => { setTimeout(updateGutter, 0); setTimeout(updateLineNumber, 0); });
function updateLineNumber() {
    const pos = editor.selectionStart; const before = editor.value.slice(0, pos); const ln = before.split('\n').length;
    lineNumberEl.textContent = ln;
}
updateGutter();

// Save / Load
document.getElementById('saveBtn').addEventListener('click', () => {
    const blob = new Blob([editor.value], { type: 'text/plain' });
    const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = 'program.pseudo'; a.click(); URL.revokeObjectURL(a.href);
});
document.getElementById('fileInput').addEventListener('change', (ev) => {
    const f = ev.target.files[0]; if (!f) return;
    const r = new FileReader(); r.onload = () => { editor.value = r.result; updateGutter(); updateLineNumber(); }; r.readAsText(f);
});
document.getElementById('clearBtn').addEventListener('click', () => { editor.value=''; updateGutter(); terminal.innerHTML=''; });

// worker messages
worker.onmessage = (ev) => {
    const msg = ev.data;
    if (msg.type === 'wasm-ready') {
        console.log('worker wasm ready', msg.exports || '');
    } else if (msg.type === 'request-input') {
        // show modal prompt to user
        lastRequestId = msg.id;
        showModalPrompt(msg.prompt);
    } else if (msg.type === 'output') {
        // append to UI or to a popup window if opened
        appendOutput(msg.text);
    } else if (msg.type === 'error') {
        appendOutput('ERROR: ' + msg.message);
    } else {
        // fallback generic text
        if (msg.text) appendOutput(msg.text);
    }
};

// send the SAB to worker
worker.postMessage({ type: 'init', controlSab: sab });

// write response into respBuf and notify worker
function writeResponseAndWake(text) {
    const enc = new TextEncoder();
    const encoded = enc.encode(text || '');
    const writeLen = Math.min(encoded.length, RESPONSE_BYTES - 1);
    respBuf.fill(0);
    respBuf.set(encoded.subarray(0, writeLen));
    Atomics.store(control, 0, 2); // ready
    Atomics.notify(control, 0, 1);
    console.log(`[main] Wrote response (len ${writeLen})`);
}

// modal handling
function showModalPrompt(promptText) {
    modalPrompt.textContent = promptText || 'Input:';
    modalInput.value = '';
    modal.style.display = 'flex';
    modalInput.focus();

    const cleanup = () => { modal.style.display = 'none'; modalOk.removeEventListener('click', onOk); modalCancel.removeEventListener('click', onCancel); };
    const onOk = () => { cleanup(); writeResponseAndWake(modalInput.value || ''); lastRequestId = null; };
    const onCancel = () => { cleanup(); writeResponseAndWake(''); lastRequestId = null; };

    modalOk.addEventListener('click', onOk);
    modalCancel.addEventListener('click', onCancel);
    modalInput.addEventListener('keydown', function onKey(e) { if (e.key === 'Enter') { e.preventDefault(); onOk(); modalInput.removeEventListener('keydown', onKey); }});
}

// append output to terminal or popup
function appendOutput(text) {
    if (currentRunWindow && !currentRunWindow.closed) {
        // write into popup document (simple append)
        currentRunWindow.document.body.appendChild(document.createElement('div')).textContent = text;
    } else {
        const div = document.createElement('div'); div.className = 'line'; div.textContent = text; terminal.appendChild(div); terminal.scrollTop = terminal.scrollHeight;
    }
}

/* Run buttons */
document.getElementById('runBtn').addEventListener('click', () => {
    currentRunWindow = null;
    terminal.innerHTML = '';
    Atomics.store(control, 0, 0); // reset state
    Atomics.notify(control, 0, 1);
    const src = editor.value;
    worker.postMessage({ type: 'run', source: src, runId: Date.now() });
});