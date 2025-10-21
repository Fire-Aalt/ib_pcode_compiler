import docsString from "../pkg/README.md";

const docsContainer = document.getElementById('docsContainer');

export async function loadReadme() {
    try {
        const marked = await import("marked");
        const rawHtml = marked.parse(docsString);
        docsContainer.innerHTML = rawHtml;

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