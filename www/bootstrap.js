import "./styles/index.css";

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

import("./src/index.js").catch(e => console.error("Error importing `index.js`:", e));