console.log("Running sqlite from Worker thread.");

importScripts("./sqlite3.js");

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

  exec(params) {
    return this.db.exec(params);
  }
}

(async () => {
  let dbManager = new DBManager(self.sqlite3InitModule);
  await dbManager.init_db();
  dbManager.migrate();

  self.onmessage = async ({ data: data }) => {
    log("message from main received in worker:", data);
    let { requestId, payload } = data;
    let { sql } = payload;

    dbManager.exec({
      sql: sql,
      callback: function (row) {
        self.postMessage({ requestId, result: row });
      },
    });
  };
})();

// to check pragma options
// PRAGMA compile_options;
