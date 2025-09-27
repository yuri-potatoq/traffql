

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
      CREATE TABLE request(
        header TEXT,
        body TEXT
      ); 
    `);
    db.exec({
      sql: 'insert into request(header, body) values (?,?)',
      bind: ['header test', '{body json}'],
    });
    
    db.exec({
      sql: 'select header, body from request',
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
    locateFile: (file) => browser.runtime.getURL(file)
  })
  .then(function (sqlite3) {
    log('Done initializing. Running demo...');
    try {
      start(sqlite3);
    } catch (e) {
      error('Exception:', e.message);
    }
  });
