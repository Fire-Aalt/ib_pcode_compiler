import { createMinimalEditor } from "./editor.js";
import { samples } from "./samples.js";
import { loadReadme } from "./docs.js";

const container = document.getElementById("editor");
const editorView = createMinimalEditor(container);

const GH_OWNER = 'Fire-Aalt';
const GH_REPO = 'ib_pcode_compiler';

const RESPONSE_BYTES = 8192;
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1);
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);
control[0] = 0;
const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

/* DOM handles */
const terminal = document.getElementById('terminal');
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



let lastRequestId = null;
let currentRunWindow = null;

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
    const body = encodeURIComponent(`**Describe the bug or feedback**\n\n**Editor snapshot:**\n\`\`\`\n${editorView.state.doc.toString()}\n\`\`\`\n\n*(Add steps to reproduce, browser, OS, WASM version, etc.)*`);
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
    setEditorCodeText(samples[v]);

    setTimeout(() => {
        sampleSelect.value = '';
    }, 150);
});


setEditorCodeText(samples["welcome"]);

function setEditorCodeText(text) {
    editorView.dispatch({
        changes: { from: 0, to: editorView.state.doc.length, insert: text },
        selection: { anchor: 0 }
    });
}

/* Save/Load/Run wiring */
saveBtn.addEventListener('click', () => {
    const blob = new Blob([editorView.state.doc.toString()], { type: 'text/plain' });
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
        setEditorCodeText(r.result);
    }; 
    r.readAsText(f);
});

runBtn.addEventListener('click', () => {
    currentRunWindow = null;
    terminal.innerHTML = '';
    Atomics.store(control, 0, 0);
    Atomics.notify(control, 0, 1);
    const src = editorView.state.doc.toString();
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
    } else if (msg.type === 'finish') {
        if (terminal.innerText.length !== 0) {
            appendOutput("\n");
        }
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
    let node = document.createElement('div');
    node.innerHTML = text;
    terminal.appendChild(node);
    terminal.scrollTop = terminal.scrollHeight;
}

loadReadme();