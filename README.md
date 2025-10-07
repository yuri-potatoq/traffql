# rWasm-Ext

### Roadmap:
- Make it available to another browsers. Currently only able to implement Sqlite3 backed by OPFS on Chrome.
    - Currently, firefox doesn't support OPFS with service worker jobs correctly.
    The reason is to use OPFS we need to claim some key into extension manifest (cross_origin_embedder_policy & cross_origin_opener_policy).
    Keys which enable headers sqlite+OPFS need beacuse it use 'SharedArrayBuffer'.
    To support crossOriginIsolated and SharedArrayBuffer, firefox should have to support multiple processes, at least for extensions that opt in to COEP/COOP.
        - https://discourse.mozilla.org/t/sqlite-and-opfs-not-working-in-firefox-extension/111667/4
        - https://bugzilla.mozilla.org/show_bug.cgi?id=1673477

### Refs:
- https://gist.github.com/573/885a062ca49d2db355c22004cc395066
- https://github.com/NixOS/nixpkgs/issues/208615
- https://webkit.org/blog/12257/the-file-system-access-api-with-origin-private-file-system/
- https://discourse.nixos.org/t/failed-to-compile-openssl-sys-with-rust-on-macos/20785/6
- https://github.com/mdn/webextensions-examples
- https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest
- https://discourse.mozilla.org/t/how-to-use-webrequest-with-manifest-v3/124008
- https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Intercept_HTTP_requests
- https://sqlite.org/wasm/doc/trunk/persistence.md
- https://betterprogramming.pub/chrome-extension-intercepting-and-reading-the-body-of-http-requests-dd9ebdf2348b?gi=c12bd15d0315
- https://www.moesif.com/blog/technical/apirequest/How-We-Captured-AJAX-Requests-with-a-Chrome-Extension/
- https://github.com/requestly/requestly
- https://github.com/DannyMoerkerke/swopr
- https://github.com/rimutaka/spotify-playlist-builder/blob/master/extension/js/background.js#L3
- https://patrickbrosset.com/articles/2023-01-17-web-storage/
- https://github.com/sqlite/sqlite/blob/master/ext/wasm/test-opfs-vfs.js
- https://github.com/sqlite/sqlite-wasm?tab=readme-ov-file#sqlite-wasm
- https://developer.chrome.com/blog/sqlite-wasm-in-the-browser-backed-by-the-origin-private-file-system
- https://groups.google.com/a/chromium.org/g/chromium-extensions/c/WsxV-5CqAko/m/pgN3ywLCCwAJ
- https://bugzilla.mozilla.org/show_bug.cgi?id=1673477
- https://github.com/randyl/sqlite3-wasm-demo-extension-mv2/tree/main
- https://stackoverflow.com/questions/9515704/access-variables-and-functions-defined-in-page-context-from-an-extension/9517879#9517879
- [use offscreen to spawn worker](https://github.com/w3c/webextensions/issues/352#issuecomment-1462105163)
- [basic offscrean example](https://github.com/rustyzone/offscreen-doc-mv3/blob/main/worker.js)
- [OPFS file system arch](https://web.dev/articles/origin-private-file-system#the_user-visible_versus_the_origin_private_file_system)
- [Notion use case with OPFS & WASM](https://www.notion.com/blog/how-we-sped-up-notion-in-the-browser-with-wasm-sqlite)
- [jq stack machine CALL execution](https://github.com/jqlang/jq/blob/master/src/execute.c#L935-L959)
- [JVM stack](https://www.artima.com/insidejvm/ed2/jvm8.html)
- [design of stack machines](https://www.researchgate.net/publication/220950780_Design_and_Implementation_of_an_Efficient_Stack_Machine)
- [rust-wasm-browser-extension](https://dev.to/rimutaka/chrome-extension-with-rust-and-wasm-by-example-5cbh)
- [Stack-Based-Architecture-and-Stack-Based-Query-Language](https://www.odbms.org/wp-content/uploads/2013/11/030.02-Subieta-Stack-Based-Architecture-and-Stack-Based-Query-Language-March-2008.pdf?utm_source=chatgpt.com)

