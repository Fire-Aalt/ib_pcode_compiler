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
const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

/* DOM handles */
const terminal = document.getElementById('terminal');
const terminalLines = document.getElementById('terminalLines');
const editor = document.getElementById('editor');
const gutter = document.getElementById('gutter');
const sampleSelect = document.getElementById('sampleSelect');

const modal = document.getElementById('modal');
const modalPrompt = document.getElementById('modalPrompt');
const modalInput = document.getElementById('modalInput');
const modalOk = document.getElementById('modalOk');
const modalCancel = document.getElementById('modalCancel');

const themeToggle = document.getElementById('themeToggle');
const reportBtn = document.getElementById('reportBtn');
const githubBtn = document.getElementById('githubBtn');
const runBtn = document.getElementById('runBtn');
const saveBtn = document.getElementById('saveBtn');
const fileLabel = document.querySelector('.file-label');

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
    applyTheme( getSystemPref() );
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

/* ---- Samples: apply and reset to default ---- */
const samples = {
    welcome: `output "Welcome"\nloop COUNT from 1 to 5\n  output COUNT\nend loop`,
    input_demo: `output "Enter something"\nA = input("Your value:")\noutput "Done: ", A`,
};


for (const k of Object.keys(samples)) {
    const opt = document.createElement('option');
    opt.value = k; opt.textContent = k;
    sampleSelect.appendChild(opt);
}
sampleSelect.addEventListener('change', () => {
    const v = sampleSelect.value;
    if (!v) return;
    editor.value = samples[v];
    updateGutter();

    // reset dropdown to the default label after a brief moment so user sees the option applied
    setTimeout(() => {
        sampleSelect.value = '';
    }, 150);
});

/* Gutter / editor logic (unchanged) */
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

/* Save/Load/Run wiring (unchanged) */
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
    terminalLines.innerHTML = '';
    Atomics.store(control, 0, 0);
    Atomics.notify(control, 0, 1);
    const src = editor.value;
    worker.postMessage({ type: 'run', source: src, runId: Date.now() });
});

/* Worker messaging (unchanged) */
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

/* Modal code (unchanged) */
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

function appendOutput(text) {
    terminalLines.innerHTML += text + '\n';
    terminal.scrollTop = terminal.scrollHeight;
}

/* README rendering + anchor scrolling inside docsContainer */
const docsContainer = document.getElementById('docsContainer');

function slugify(text) {
    return text
        .toString()
        .toLowerCase()
        .trim()
        .replace(/\s+/g, '-')       // spaces -> dashes
        .replace(/[^\w\-]+/g, '')   // remove non-word chars
}

function scrollToIdInDocs(id, { behavior = 'smooth', offset = 8 } = {}) {
    if (!id) return false;
    // use CSS.escape for safety
    const selector = '#' + CSS.escape(id);
    const target = docsContainer.querySelector(selector);
    if (!target) return false;

    // compute scrollTop relative to docsContainer
    const containerRect = docsContainer.getBoundingClientRect();
    const targetRect = target.getBoundingClientRect();
    const relativeTop = targetRect.top - containerRect.top + docsContainer.scrollTop;

    docsContainer.scrollTo({
        top: Math.max(0, relativeTop - offset),
        behavior,
    });
    return true;
}

function attachDocsAnchorHandler() {
    // Intercept clicks on links inside docsContainer
    docsContainer.addEventListener('click', (ev) => {
        const a = ev.target.closest('a');
        if (!a) return;
        const href = a.getAttribute('href') || '';

        // #fragment links
        if (href.startsWith('#')) {
            ev.preventDefault();
            const id = href.slice(1);
            if (scrollToIdInDocs(id)) {
                // update url hash without native jump
                try { history.replaceState(null, '', href); } catch (e) {}
            }
            return;
        }

        // links with same-page + hash (e.g. /path/page.html#section or full URL)
        try {
            const url = new URL(href, location.href);
            const isSamePage = (url.pathname === location.pathname && url.search === location.search);
            if (isSamePage && url.hash) {
                ev.preventDefault();
                const id = url.hash.slice(1);
                if (scrollToIdInDocs(id)) {
                    try { history.replaceState(null, '', url.hash); } catch (e) {}
                }
            }
        } catch (err) {
            // ignore invalid URLs
        }
    });
}

async function loadReadme() {
    try {
        const resp = await fetch('./pkg/README.md');
        if (!resp.ok) {
            docsContainer.innerHTML = `<p class="muted">README.md not found: ${resp.status}</p>`;
            return;
        }
        const md = await resp.text();

        const rawHtml = window.marked.parse(md);
        docsContainer.innerHTML = rawHtml.replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, '');

        // Ensure headings have stable IDs (use existing id if present)
        docsContainer.querySelectorAll('h1,h2,h3,h4,h5,h6').forEach(h => {
            if (!h.id) {
                const candidate = slugify(h.textContent || h.innerText || 'section');
                // ensure uniqueness
                let id = candidate;
                let i = 1;
                while (docsContainer.querySelector('#' + CSS.escape(id))) {
                    id = `${candidate}-${i++}`;
                }
                h.id = id;
            }
        });

        // Attach click handler once (idempotent)
        if (!docsContainer._anchorsAttached) {
            attachDocsAnchorHandler();
            docsContainer._anchorsAttached = true;
        }

        // If the current page URL already has a hash, scroll to it after rendering
        const currentHash = window.location.hash;
        if (currentHash) {
            // small timeout to allow layout / fonts to settle
            setTimeout(() => {
                scrollToIdInDocs(currentHash.slice(1), { behavior: 'auto', offset: 8 });
            }, 60);
        }
    } catch (err) {
        docsContainer.innerHTML = `<p class="muted">Failed to load docs: ${err.message}</p>`;
    }
}

await loadReadme();