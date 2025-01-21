console.log('Running demo from Worker thread.');

const log = console.log;
const warn = console.log;
const error = console.log;

// chrome.runtime.onMessage.addListener( (request, sender, sendResponse) => {
//   console.log(`request: ${request} sender: ${sender} sendResp: ${sendResponse}`);
    
// });


self.onmessage = (msg) => {
  console.log("message from main received in worker:", msg);
};


const start = function (sqlite3) {
  const capi = sqlite3.capi; /*C-style API*/
  const oo = sqlite3.oo1; /*high-level OO API*/
  log('sqlite3 version', capi.sqlite3_libversion(), capi.sqlite3_sourceid());
  let db;
  if (sqlite3.opfs) {
    db = new oo.OpfsDb('/mydb.sqlite3');
    log('The OPFS is available.');
  } else {
    db = new oo.DB('/mydb.sqlite3', 'ct');
    log('The OPFS is not available.');
  }
  log('transient db =', db.filename);

  try {
    log('all enabled extensions');
    db.exec({
      sql: 'PRAGMA compile_options;',
      rowMode: 'array',
      callback: function (row) {
        log(`${row}`);
      },
    })
    
    log('Create a table...');
    db.exec(`
      CREATE TABLE IF NOT EXISTS requests_headers(
          header_id INTEGER PRIMARY KEY,
          key TEXT NOT NULL,
          value TEXT NOT NULL
      );
      
      CREATE TABLE IF NOT EXISTS requests_url(
          url_id INTEGER PRIMARY KEY,
          protocol TEXT NOT NULL,
          domain TEXT NOT NULL,
          path TEXT NOT NULL
      );
      
      CREATE TABLE IF NOT EXISTS requests_json_body(
          json_body_id INTEGER PRIMARY KEY,
          body JSONB NOT NULL
      );
      
      CREATE TABLE IF NOT EXISTS requests(
          req_id INTEGER PRIMARY KEY,
          url_fk INTEGER NOT NULL,
          header_fk INTEGER NOT NULL,
          json_body_fk INTEGER NOT NULL,
      
          FOREIGN KEY(url_fk) REFERENCES requests_url(url_id),
          FOREIGN KEY(header_fk) REFERENCES requests_headers(header_id),
          FOREIGN KEY(json_body_fk) REFERENCES requests_json_body(json_body_id)
      );

    `);
    // db.exec({
    //   sql: 'insert into t(a,b) values (?,?)',
    //   bind: [i, i * 2],
    // });
    
    db.exec({
      sql: 'select a from t order by a limit 3',
      rowMode: 'array', // 'array' (default), 'object', or 'stmt'
      callback: function (row) {
        log('row ', ++this.counter, '=', row);
      }.bind({ counter: 0 }),
    });
  } finally {
    db.close();
  }
};

importScripts('./sqlite3.js');

self
  .sqlite3InitModule({
    print: log,
    printErr: error,
  })
  .then(function (sqlite3) {
    log('Done initializing. Running demo...');
    try {
      start(sqlite3);
    } catch (e) {
      error('Exception:', e.message);
    }
  });
