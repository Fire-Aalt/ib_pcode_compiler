// main.js (type=module)
// -- service worker registration unchanged
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

const GH_OWNER = 'Fire-Aalt';
const GH_REPO = 'ib_pcode_compiler';

const RESPONSE_BYTES = 8192;
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1);
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);
control[0] = 0;
const worker = new Worker(new URL('./modules/worker.js', import.meta.url), { type: 'module' });

/* DOM handles */
const terminal = document.getElementById('terminal');
const editor = document.getElementById('editor');
const gutter = document.getElementById('gutter');
const sampleSelect = document.getElementById('sampleSelect');

const modal = document.getElementById('modal');
const modalPrompt = document.getElementById('modalPrompt');
const modalInput = document.getElementById('modalInput');
const modalOk = document.getElementById('modalOk');

const themeToggle = document.getElementById('themeToggle');
const reportBtn = document.getElementById('reportBtn');
const githubBtn = document.getElementById('githubBtn');
const runBtn = document.getElementById('runBtn');
const saveBtn = document.getElementById('saveBtn');
const fileLabel = document.querySelector('.file-label');

const docsContainer = document.getElementById('docsContainer');

let lastRequestId = null;
let currentRunWindow = null;

/* Tabs: update toolbar visibility on tab change */
function setActiveTab(tabName) {
    // tabName is 'editor'|'docs' etc.
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    const btn = document.querySelector(`.tab[data-tab="${tabName}"]`);
    if (btn) btn.classList.add('active');

    document.querySelectorAll('.tab-panel').forEach(p => p.hidden = true);
    const panel = document.getElementById(tabName + 'Tab');
    if (panel) panel.hidden = false;

    // hide toolbar controls when docs tab active
    const hide = (tabName === 'docs');
    sampleSelect.style.display = hide ? 'none' : '';
    saveBtn.style.display = hide ? 'none' : '';
    fileLabel.style.display = hide ? 'none' : '';
    runBtn.style.display = hide ? 'none' : '';
}

// wire tab buttons
document.querySelectorAll('.tab').forEach(btn => {
    btn.addEventListener('click', () => {
        // report button may not have data-tab – handle separately
        const tab = btn.dataset.tab;
        if (tab) setActiveTab(tab);
    });
});

// initialize active tab on load
document.addEventListener('DOMContentLoaded', () => {
    setActiveTab('editor');
});

/* Theme toggle (follows system unless user chooses) */
const THEME_KEY = 'ibp-theme';
function getSystemPref() {
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme) {
    if (theme === 'dark') {
        document.body.setAttribute('data-theme', 'dark');
        themeToggle.textContent = '☀️';
        themeToggle.title = 'Switch to light theme';
    } else {
        document.body.removeAttribute('data-theme');
        themeToggle.textContent = '🌙';
        themeToggle.title = 'Switch to dark theme';
    }
}

const savedTheme = localStorage.getItem(THEME_KEY);

if (savedTheme === 'dark' || savedTheme === 'light') {
    applyTheme(savedTheme);
} else {
    applyTheme(getSystemPref());
    // follow system if user hasn't chosen
    if (window.matchMedia) {
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        mq.addEventListener('change', (ev) => {
            if (!localStorage.getItem(THEME_KEY)) applyTheme(ev.matches ? 'dark' : 'light');
        })
    }
}
themeToggle.addEventListener('click', () => {
    const newTheme = document.body.hasAttribute('data-theme') ? 'light' : 'dark';
    applyTheme(newTheme);
    localStorage.setItem(THEME_KEY, newTheme);
});

/* Report button opens GitHub issues directly (prefills snapshot) */
reportBtn.addEventListener('click', () => {
    const title = encodeURIComponent('Bug report: [short description]');
    const body = encodeURIComponent(`**Describe the bug or feedback**\n\n**Editor snapshot:**\n\`\`\`\n${editor.value}\n\`\`\`\n\n*(Add steps to reproduce, browser, OS, WASM version, etc.)*`);
    const url = `https://github.com/${GH_OWNER}/${GH_REPO}/issues/new?title=${title}&body=${body}`;
    window.open(url, '_blank', 'noopener');
});

githubBtn.addEventListener('click', () => {
    const url = `https://github.com/${GH_OWNER}/${GH_REPO}`;
    window.open(url, '_blank', 'noopener');
});

function nicifyKey(key) {
    if (!key) return key;
    let s = key.replace(/[_-]+/g, ' ');
    s = s.replace(/([a-z0-9])([A-Z])/g, '$1 $2');
    s = s.replace(/([A-Z])([A-Z][a-z])/g, '$1 $2');
    s = s.replace(/\s+/g, ' ').trim();
    s = s.split(' ').map(w => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase()).join(' ');
    return s;
}

for (const k of Object.keys(samples)) {
    const opt = document.createElement('option');
    opt.value = k;
    opt.textContent = nicifyKey(k);
    sampleSelect.appendChild(opt);
}

sampleSelect.addEventListener('change', () => {
    const v = sampleSelect.value;
    if (!v) return;
    editor.value = samples[v];
    updateGutter();

    setTimeout(() => {
        sampleSelect.value = '';
    }, 150);
});

editor.value = samples["welcome"];
updateGutter();


/* Gutter / editor logic */
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

editor.addEventListener('keydown', (e) => {
    if (e.key !== 'Tab') {
        setTimeout(updateGutter, 0);
        return;
    }
    e.preventDefault();

    const start = editor.selectionStart;
    const end = editor.selectionEnd;
    const value = editor.value;
    const tabChar = '\t';

    // Line numbers for selection
    const selStartLine = value.slice(0, start).split('\n').length - 1;
    const selEndLine = value.slice(0, end).split('\n').length - 1;

    // compute line start indices
    const lines = value.split('\n');
    const lineStartIndices = [];
    let acc = 0;
    for (let i = 0; i < lines.length; i++) {
        lineStartIndices.push(acc);
        acc += lines[i].length + 1; // +1 for newline
    }

    // Multi-line indent/unindent
    if (selStartLine !== selEndLine || (start !== end && value.slice(start, end).includes('\n'))) {
        const isUnindent = e.shiftKey;

        // Build replacement for the exact block from first affected line start
        const blockStart = lineStartIndices[selStartLine];
        const blockEnd = lineStartIndices[selEndLine] + lines[selEndLine].length; // exclusive end index

        // Extract the block lines and modify them
        const blockText = value.slice(blockStart, blockEnd);
        const blockLines = blockText.split('\n');

        for (let li = 0; li < blockLines.length; li++) {
            if (!isUnindent) {
                blockLines[li] = tabChar + blockLines[li];
            } else {
                if (blockLines[li].startsWith('\t')) blockLines[li] = blockLines[li].slice(1);
                else if (blockLines[li].startsWith('    ')) blockLines[li] = blockLines[li].slice(4);
            }
        }

        const replacement = blockLines.join('\n');
        
        // Replace only the block range so undo works properly
        editor.setRangeText(replacement, blockStart, blockEnd, 'select');

        // After 'select' the selection is the inserted block; adjust to expected selection (same lines)
        const newSelStart = blockStart;
        const newSelEnd = blockStart + replacement.length;

        editor.selectionStart = newSelStart;
        editor.selectionEnd = newSelEnd;

        // notify listeners and update gutter
        editor.dispatchEvent(new Event('input', { bubbles: true }));
        setTimeout(updateGutter, 0);
        return;
    }

    // Single-caret insertion (no line breaks in selection)
    // Use setRangeText to insert tab and preserve undo
    editor.setRangeText(tabChar, start, end, 'end');

    // ensure caret placed after inserted tab
    const caret = start + tabChar.length;
    editor.selectionStart = editor.selectionEnd = caret;

    editor.dispatchEvent(new Event('input', { bubbles: true }));
    setTimeout(updateGutter, 0);
});


/* Save/Load/Run wiring */
saveBtn.addEventListener('click', () => {
    const blob = new Blob([editor.value], { type: 'text/plain' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'program.pseudo';
    a.click();
    URL.revokeObjectURL(a.href);
});

document.getElementById('fileInput').addEventListener('change', (ev) => {
    const f = ev.target.files[0]; 
    if (!f) return;
    const r = new FileReader(); 
    r.onload = () => { 
        editor.value = r.result;
        updateGutter();
    }; 
    r.readAsText(f);
});

runBtn.addEventListener('click', () => {
    currentRunWindow = null;
    terminal.innerHTML = '';
    Atomics.store(control, 0, 0);
    Atomics.notify(control, 0, 1);
    const src = editor.value;
    worker.postMessage({ type: 'run', source: src, runId: Date.now() });
});

/* Worker messaging */
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

function writeResponseAndWake(text) {
    const enc = new TextEncoder();
    const encoded = enc.encode(text || '');
    const writeLen = Math.min(encoded.length, RESPONSE_BYTES - 1);
    respBuf.fill(0);
    respBuf.set(encoded.subarray(0, writeLen));
    Atomics.store(control, 0, 2);
    Atomics.notify(control, 0, 1);
}

function showModalPrompt(promptText) {
    modalPrompt.textContent = promptText || 'Input:';
    modalInput.value = '';
    modal.style.display = 'flex';
    modalInput.focus();

    const cleanup = () => { 
        modal.style.display = 'none';
        modalOk.removeEventListener('click', onOk);
    };
    const onOk = () => { 
        cleanup();
        writeResponseAndWake(modalInput.value || '');
        lastRequestId = null;
    };

    modalOk.addEventListener('click', onOk);
    modalInput.addEventListener('keydown', function onKey(e) { 
        if (e.key === 'Enter') { 
            e.preventDefault();
            onOk();
            modalInput.removeEventListener('keydown', onKey);
        }
    });
}

function appendOutput(text) {
    terminal.innerHTML += text + '\n';
    terminal.scrollTop = terminal.scrollHeight;
}

await loadReadme();