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