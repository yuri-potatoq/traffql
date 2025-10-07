console.log("Running sqlite from Worker thread.");

import __wbg_init, { QueryEngine } from "./traffQL.js";

import { default as sqlite3InitModule } from "./sqlite3-bundler-friendly.mjs";

const log = console.log;
const warn = console.log;
const error = console.log;

class DBManager {
  constructor(sqlite3InitModule) {
    this.sqlite3InitModule = sqlite3InitModule;
    this.db = null;
  }

  init_db() {
    return this.sqlite3InitModule({
      print: log,
      printErr: error,
    }).then((sqlite3) => {
      log("Done initializing. Running demo...");
      try {
        const capi = sqlite3.capi; /*C-style API*/
        const oo = sqlite3.oo1; /*high-level OO API*/
        log(
          "sqlite3 version",
          capi.sqlite3_libversion(),
          capi.sqlite3_sourceid(),
        );

        // instance of sqlite DB is equivalent to 'sqlite3_open()'
        if (sqlite3.opfs) {
          this.db = new oo.OpfsDb("/traffql.sqlite3");
          log("The OPFS is available.");
        } else {
          this.db = new oo.DB("/traffql.sqlite3", "ct");
          log("The OPFS is not available.");
        }
        log("transient db =", this.db.filename);
      } catch (e) {
        error("Exception:", e.message);
      }
    });
  }

  migrate() {
    this.db.exec(`
      CREATE TABLE IF NOT EXISTS request(
        url TEXT,
        response_body TEXT,
        response_code TEXT
      );
    `);
  }

  async query(sql) {
    try {
      const resultRows = [];
      this.db.exec({
        sql: sql,
        bind: [],
        resultRows,
        rowMode: "object",
        returnValue: "resultRows",
      });
      return resultRows;
    } catch (error) {
      throw error;
    }
  }

  exec(params) {
    return this.db.exec(params);
  }
}

(async () => {
  let dbManager = new DBManager(sqlite3InitModule);
  await dbManager.init_db();
  dbManager.migrate();

  await __wbg_init();

  self.onmessage = async ({ data: data }) => {
    log("message from main received in worker:", data);

    const queryEngine = new QueryEngine(dbManager);

    await handleMessage.bind({
      db: dbManager,
      queryEngine,
    })(data, self.postMessage);
  };
})();

async function handleMessage({ requestId, kind, payload }, replyCallback) {
  switch (kind) {
    case "dump":
      //TODO: improve to export/import dbs
      // https://sqlite.org/wasm/doc/trunk/cookbook.md#impexp
      log("[WORKER]: writing file handler");
      let { handle } = payload;

      const root = await navigator.storage.getDirectory();
      const fileHandle = await root.getFileHandle("traffql.sqlite3");
      const file = await fileHandle.getFile();
      const buf = await file.arrayBuffer();

      const writable = await handle.createWritable();
      await writable.write(buf);
      await writable.close();

      log("[WORKER]: writing file finished!");
      replyCallback({ requestId, result: "finished" });
      break;
    case "ql":
      let { query } = payload;

      log("[WORKER]: received query", query);
      replyCallback({ requestId, result: query });
      let result = this.queryEngine.execute_query("select 'request.status_code'");
      log("[WORKER][query_result]: ", result);

      break;
    case "fetch-data":
      let { request, response } = payload;
      let { code, headers: resHeaders, body: resBody } = response;
      let { url, headers: reqHeaders, method, body: reqBody } = request;

      replyCallback({ requestId, result: payload });
      this.db.exec({
        sql: "insert into request(url) values (?);",
        bind: [url],
      });
      log("[WORKER]: handle fetch data:", payload);
      break;
    default:
      error("[WORKER]: can't handle message kind:", kind);
      replyCallback({ requestId, error: "not know kind" });
  }
}

// to check pragma options
// PRAGMA compile_options;
