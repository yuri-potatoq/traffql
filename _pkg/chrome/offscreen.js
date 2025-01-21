console.log(`Hello From Offscreen!`)

const worker = new Worker(chrome.runtime.getURL('worker.js'));


chrome.runtime.onMessageExternal.addListener( (request, sender, sendResponse) => {
  worker.postMessage({requestData: request.requestData, responseData: request.responseData})  
});


// chrome.runtime.onMessage.addListener( (request, sender, sendResponse) => {
//   console.log(`request: ${request} sender: ${sender} sendResp: ${sendResponse}`);
//   worker.postMessage("global", {request: request, sender: sender, sendResponse: sendResponse})  
// });