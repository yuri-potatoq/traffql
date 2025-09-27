const port = chrome.runtime.connect({ name: "devtools-db" });

// map requestId -> {resolve,reject}
const pending = new Map();
let nextId = 1;

port.onMessage.addListener((msg) => {
  if (!msg || !msg.requestId) return;

  const p = pending.get(msg.requestId);
  if (!p) return;

  pending.delete(msg.requestId);
  if (msg.error) {
    p.reject(new Error(msg.error));
  } else {
    p.resolve(msg.result);
  }
});

function sendDbRequest(payload, timeoutMs = 10000) {
  const requestId = `r${nextId++}`;
  const message = { requestId, kind: "db", payload }; // payload: { op:'select', sql, params }

  return new Promise((resolve, reject) => {
    pending.set(requestId, { resolve, reject });
    port.postMessage(message);

    const t = setTimeout(() => {
      if (pending.has(requestId)) {
        pending.delete(requestId);
        reject(new Error("DB request timeout"));
      }
    }, timeoutMs);

    // clear timeout when resolved/rejected:
    const origResolve = resolve;
    resolve = (v) => {
      clearTimeout(t);
      origResolve(v);
    };
  });
}

const queryForm = document.getElementById("query-form");
queryForm.addEventListener("submit", async function (evt) {
  evt.preventDefault();
  const formData = new FormData(queryForm);
  const query = formData.get("query");

  try {
    const rows = await sendDbRequest({
      op: "select",
      sql: query,
    });
    console.log("rows", rows);
  } catch (err) {
    console.error("db err", err);
  }
});
