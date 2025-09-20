pub(crate) const DB_SCEMA: &str = r#"
CREATE TABLE IF NOT EXISTS accounts (
    primary_id  TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    currency    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS categories (
    primary_id  TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS transactions (
    primary_id      TEXT PRIMARY KEY NOT NULL,
    amount          INTEGER NOT NULL,
    date            TEXT NOT NULL,
    category_id     TEXT,
    currency_rate   REAL,
    from_id         TEXT,
    to_id           TEXT,

    FOREIGN KEY (category_id) REFERENCES categories (primary_id),
    FOREIGN KEY (from_id) REFERENCES accounts (primary_id),
    FOREIGN KEY (to_id) REFERENCES accounts (primary_id)
);
"#;

pub(crate) const ACCOUNT_SET: &str =
    "INSERT INTO accounts (primary_id, name, description, currency) VALUES (?1, ?2, ?3, ?4)";

pub(crate) const ACCOUNTS_GET: &str =
    "SELECT primary_id, name, description, currency FROM accounts";
