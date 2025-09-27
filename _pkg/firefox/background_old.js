import init_wasm, { save_incomming_request, RequestDetails } from './traffiQL.js';
import './sqlite-wasm/jswasm/sqlite3-worker1-promiser.mjs';

const sqlite3Worker1Promiser = self.sqlite3Worker1Promiser;

// TODO: currently, firefox doesn't support OPFS with service worker jobs correctly.
// The reason is to use OPFS we need to claim some key into extension manifest (cross_origin_embedder_policy & cross_origin_opener_policy).
// Keys which enable headers sqlite+OPFS need beacuse it use 'SharedArrayBuffer'.
//
// https://discourse.mozilla.org/t/sqlite-and-opfs-not-working-in-firefox-extension/111667/4
// https://bugzilla.mozilla.org/show_bug.cgi?id=1673477

(async () => {
    await init_wasm(); 
})();

// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Match_patterns#examples
const wildCardPattern = "<all_urls>";


const log = console.log;
const error = console.error;

log("TRY LOADING BACKGROUND SCRIPT!");
log(`crossOriginIsolated: ${Window.crossOriginIsolated}`)

async function listenerM(details) {
    try {
        log(details.documentUrl);

        log(`--------------------RESPONSE ---------------------------`)

        let filter = browser.webRequest.filterResponseData(details.requestId);
        let decoder = new TextDecoder("utf-8");
        let encoder = new TextEncoder();
        
        filter.ondata = event => {
            let req = new RequestDetails();

            let str = decoder.decode(event.data, { stream: true });
            filter.write(encoder.encode(str));
            filter.disconnect();

            log(`-----------------------------------------------`)
            log(`DOC_URL: ${details.url}`)
            log(`REQ: TYPE: ${details.type}`)
            log(`REQ: ID: ${details.requestId}`)
            log(`RESP BODY: ${str}`)

            //let headers = "".concat(details.requestHeaders.map( h => `${h.name}: ${h.value}`));
            //log(`REQ: HEADERS: ${ headers }`)

            req.ID = details.requestId;
            req.target_url = details.url;
            req.headers = ['Test Header'];
            req.raw_body = str;
            save_incomming_request(req);
            
        }
    } catch (error) {
        console.error(error)
    }

    return {};
}


/**
 * webRequest.onBeforeRequest:
 *  - requestId # we need to correlation with another steps
 *  - request body
 *  - request method
 * 
 * webRequest.onBeforeSendHeaders:
 *  - requestId
 *  - request method
 *  - request headers
 * 
 * webRequest.onCompleted:
 *  - requestId
 *  - response header
 *  - response status code
 *  - response type: webRequest.ResourceType is a string.
 *    - in case of xmlhttprequest we CAN'T extract the response body from webRequest events. We need to workaround and create a wrapper of XMLHttpRequest.prototype to handle it.
 *    - https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/ResourceType#xmlhttprequest
 *    - https://stackoverflow.com/a/71012727
 *    - don't use devtools extensions cause only works when it is open
 */

const addListenerBeForeRequestArgs = {
    listener: listenerM,
    // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/RequestFilter
    filter: {
        urls: [wildCardPattern], 
        
        // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/ResourceType
        types: ["main_frame", "xmlhttprequest"]
    },
    extraInfoSpec: [
        "blocking",
        //"requestBody"
    ]
}



browser.webRequest.onBeforeSendHeaders.addListener(
    addListenerBeForeRequestArgs.listener,
    addListenerBeForeRequestArgs.filter,
    addListenerBeForeRequestArgs.extraInfoSpec
);



const initializeSQLite = async () => {
  try {
    log('Loading and initializing SQLite3 module...');

    const promiser = await new Promise((resolve) => {
      const _promiser = sqlite3Worker1Promiser({
        onready: () => resolve(_promiser),
      });
    });

    log('Done initializing. Running demo...');

    const configResponse = await promiser('config-get', {});
    log('Running SQLite3 version', configResponse.result.version.libVersion);

    const openResponse = await promiser('open', {
      filename: 'file:mydb.sqlite3?vfs=opfs',
    });
    const { db } = openResponse;
    log(
      'OPFS is available, created persisted database at',
      openResponse.result.filename.replace(/^file:(.*?)\?vfs=opfs$/, '$1'),
    );
    
    try {
        log('Creating a table...');
        db.exec('CREATE TABLE IF NOT EXISTS t(a,b)');
        log('Insert some data using exec()...');
        for (let i = 20; i <= 25; ++i) {
          db.exec({
            sql: 'INSERT INTO t(a,b) VALUES (?,?)',
            bind: [i, i * 2],
          });
        }
        log('Query data with exec()...');
        db.exec({
          sql: 'SELECT a FROM t ORDER BY a LIMIT 3',
          callback: (row) => {
            log(row);
          },
        });
      } finally {
        db.close();
      }
  } catch (err) {
    if (!(err instanceof Error)) {
      err = new Error(err.result.message);
    }
    error(err.name, err.message);
  }
};

initializeSQLite();

log("END BACKGROUND SCRIPT!")