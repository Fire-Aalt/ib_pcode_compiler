// main.js (type=module)
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

const RESPONSE_BYTES = 8192; // must match worker side
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1);
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);
control[0] = 0;

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

const terminal = document.getElementById('terminal');
const editor = document.getElementById('editor');
const gutter = document.getElementById('gutter');
const sampleSelect = document.getElementById('sampleSelect');

const modal = document.getElementById('modal');
const modalPrompt = document.getElementById('modalPrompt');
const modalInput = document.getElementById('modalInput');
const modalOk = document.getElementById('modalOk');
const modalCancel = document.getElementById('modalCancel');

let lastRequestId = null;
let currentRunWindow = null;

// Tab UI
document.querySelectorAll('.tab').forEach(btn => {
    btn.addEventListener('click', () => {
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        btn.classList.add('active');
        const tab = btn.dataset.tab;
        document.querySelectorAll('.tab-panel').forEach(p => p.hidden = true);
        document.getElementById(tab + 'Tab').hidden = false;
    });
});

// samples
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
    editor.value = samples[v]; updateGutter();
});

// gutter and line sync
function updateGutter() {
    const lines = Math.max(1, editor.value.split('\n').length);
    gutter.innerHTML = '';
    const cs = getComputedStyle(editor);
    let lineHeight = parseFloat(cs.lineHeight);
    if (!Number.isFinite(lineHeight) || lineHeight === 0) lineHeight = 20;
    for (let i = 1; i <= lines; i++) {
        const d = document.createElement('div');
        d.textContent = i.toString();
        d.style.height = lineHeight + 'px';
        d.style.lineHeight = lineHeight + 'px';
        gutter.appendChild(d);
    }
}
editor.addEventListener('input', () => updateGutter());
editor.addEventListener('scroll', () => { gutter.scrollTop = editor.scrollTop; terminal.scrollTop = terminal.scrollHeight; });

// Tab / shift-tab & insert tab
editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        const value = editor.value;
        const tabChar = '\t';
        const selStartLine = value.slice(0, start).split('\n').length - 1;
        const selEndLine = value.slice(0, end).split('\n').length - 1;

        if (selStartLine !== selEndLine || (start !== end && value.slice(start, end).includes('\n'))) {
            // multi-line indent/unindent
            const lines = value.split('\n');
            let lineStartIndices = [];
            let acc = 0;
            for (let i = 0; i < lines.length; i++) {
                lineStartIndices.push(acc);
                acc += lines[i].length + 1;
            }
            const isUnindent = e.shiftKey;
            for (let li = selStartLine; li <= selEndLine; li++) {
                if (!isUnindent) lines[li] = tabChar + lines[li];
                else {
                    if (lines[li].startsWith('\t')) lines[li] = lines[li].slice(1);
                    else if (lines[li].startsWith('    ')) lines[li] = lines[li].slice(4);
                }
            }
            const newValue = lines.join('\n');
            const newStart = lineStartIndices[selStartLine] + (isUnindent ? 0 : tabChar.length);
            const newEnd = lineStartIndices[selEndLine] + lines[selEndLine].length + 1;
            editor.value = newValue;
            editor.selectionStart = newStart;
            editor.selectionEnd = newEnd - 1;
            setTimeout(updateGutter, 0);
            return;
        }

        // single caret
        const before = value.slice(0, start);
        const after = value.slice(end);
        editor.value = before + tabChar + after;
        const caret = start + tabChar.length;
        editor.selectionStart = editor.selectionEnd = caret;
        setTimeout(updateGutter, 0);
        return;
    }
    setTimeout(updateGutter, 0);
});
updateGutter();

// Save / Load
document.getElementById('saveBtn').addEventListener('click', () => {
    const blob = new Blob([editor.value], { type: 'text/plain' });
    const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = 'program.pseudo'; a.click(); URL.revokeObjectURL(a.href);
});
document.getElementById('fileInput').addEventListener('change', (ev) => {
    const f = ev.target.files[0]; if (!f) return;
    const r = new FileReader(); r.onload = () => { editor.value = r.result; updateGutter(); }; r.readAsText(f);
});

// worker messaging
worker.onmessage = (ev) => {
    const msg = ev.data;
    if (msg.type === 'wasm-ready') {
        console.log('wasm-ready', msg);
    } else if (msg.type === 'request-input') {
        lastRequestId = msg.id;
        showModalPrompt(msg.prompt);
    } else if (msg.type === 'output') {
        appendOutput(msg.text);
    } else if (msg.type === 'error') {
        appendOutput('ERROR: ' + msg.message);
    } else {
        if (msg.text) appendOutput(msg.text);
    }
};
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

// append output
function appendOutput(text) {
    if (currentRunWindow && !currentRunWindow.closed) {
        currentRunWindow.document.body.appendChild(document.createElement('div')).textContent = text;
    } else {
        const div = document.createElement('div'); div.className = 'line'; div.textContent = text; terminal.appendChild(div); terminal.scrollTop = terminal.scrollHeight;
    }
}

// Run button
document.getElementById('runBtn').addEventListener('click', () => {
    currentRunWindow = null;
    terminal.innerHTML = '';
    Atomics.store(control, 0, 0); // reset state
    Atomics.notify(control, 0, 1);
    const src = editor.value;
    worker.postMessage({ type: 'run', source: src, runId: Date.now() });
});

// README rendering
const docsContainer = document.getElementById('docsContainer');
async function loadReadme() {
    try {
        const resp = await fetch('./pkg/README.md');
        if (!resp.ok) { docsContainer.innerHTML = `<p class="muted">README.md not found: ${resp.status}</p>`; return; }
        const md = await resp.text();
        docsContainer.innerHTML = marked.parse(md);
    } catch (err) {
        docsContainer.innerHTML = `<p class="muted">Failed to load docs: ${err.message}</p>`;
    }
}
loadReadme();

// Report issue: open GitHub issues with prefilled title/body
const GH_OWNER = 'your-org-or-username';
const GH_REPO = 'your-repo';
document.getElementById('openIssueBtn').addEventListener('click', () => {
    const title = encodeURIComponent(document.getElementById('issueTitle').value || 'Bug report: [short description]');
    // Prefill body: include editor content and some metadata
    const body = encodeURIComponent(`**Describe the bug or feedback**\n\n\n**Editor snapshot:**\n\`\`\`\n${editor.value}\n\`\`\`\n\n*(Add steps to reproduce, browser, OS, WASM version, etc.)*`);
    const url = `https://github.com/${GH_OWNER}/${GH_REPO}/issues/new?title=${title}&body=${body}`;
    window.open(url, '_blank', 'noopener');
});
document.getElementById('copySnapshot').addEventListener('click', async () => {
    try {
        await navigator.clipboard.writeText(editor.value);
        alert('Editor contents copied to clipboard');
    } catch {
        alert('Copy failed — please select and copy manually');
    }
});
