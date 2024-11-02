// import { save_incomming_request } from "./traffiQL";


// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Match_patterns#examples
const wildCardPattern = "<all_urls>";

// function logURL(requestDetails) {
//     console.log(`Loading: ${requestDetails.url}`);
// }


// function logNavigation(requestDetails) {
//     console.log(`NAvigation: ${requestDetails.url}`);
// }

// browser.webRequest.onBeforeRequest.addListener(logURL, {
//     urls: [wildCardPattern],
// });

// browser.webNavigation.onBeforeNavigate.addListener(logNavigation, {
//     url: [{ urlMatches: '.*' }],
// })


function log(msg) {
    console.log(`RUST_WASM_LOG MSG: ${msg}`)
}


function listenerM(details) {
    // let req = RequestDetails.new();
    log(`-----------------------------------------------`)
    log(`DOC_URL: ${details.url}`)
    log(`REQ: TYPE: ${details.type}`)
    log(`REQ: ID: ${details.requestId}`)
    // let headers = "".concat(details.requestHeaders.map( h => `${h.name}: ${h.value}`));
    // log(`REQ: HEADERS: ${ headers }`)

    // req.ID = details.requestId;
    // req.target_url = details.url;
    // req.headers = details.headers;
    // save_incomming_request(req);

    log(`-----------------------------------------------`)

    // let filter = browser.webRequest.filterResponseData(details.requestId);
    // let decoder = new TextDecoder("utf-8");
    // let encoder = new TextEncoder();

    // filter.ondata = event => {
    //     let str = decoder.decode(event.data, { stream: true });
    //     // Just change any instance of Example in the HTTP response
    //     // to WebExtension Example.
    //     str = str.replace(/Example/g, 'WebExtension Example');
    //     filter.write(encoder.encode(str));
    //     filter.disconnect();
    // }

    return {};
}

const addListenerBeForeRequestArgs = {
    listener: listenerM,
    // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/RequestFilter
    filter: {
        urls: [wildCardPattern], 
        
        // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/webRequest/ResourceType
        types: ["main_frame", "xmlhttprequest"]
    },
    extraInfoSpec: [
        // "blocking"
        // "requestBody"
    ]
}



// browser.webRequest.onBeforeSendHeaders.addListener(
//     addListenerBeForeRequestArgs.listener,
//     addListenerBeForeRequestArgs.filter,
//     addListenerBeForeRequestArgs.extraInfoSpec
// );

log("INIT BACKGROUND SCRIPT!")