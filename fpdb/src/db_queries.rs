pub(crate) const DB_SCEMA: &str = r#"
CREATE TABLE IF NOT EXISTS accounts (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    currency    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS categories (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS transactions (
    id              INTEGER PRIMARY KEY,
    amount          INTEGER NOT NULL,
    date            TEXT NOT NULL,
    category_id     TEXT,
    currency_rate   REAL,
    from_id         TEXT,
    to_id           TEXT,

    FOREIGN KEY (category_id) REFERENCES categories (id),
    FOREIGN KEY (from_id) REFERENCES accounts (id),
    FOREIGN KEY (to_id) REFERENCES accounts (id)
);
"#;

pub(crate) const ACCOUNT_SET: &str =
    "INSERT INTO accounts (name, description, currency) VALUES (?1, ?2, ?3)";

pub(crate) const ACCOUNTS_GET: &str = "SELECT name, description, currency FROM accounts";

pub(crate) const CATEGORY_SET: &str = "INSERT INTO categories (name, description) VALUES (?1, ?2)";

pub(crate) const CATEGORIES_GET: &str = "SELECT name, description FROM categories";

pub(crate) const TRANSACTION_SET: &str = r#"
INSERT INTO transactions 
    (amount, date, category_id, currency_rate, from_id, to_id) 
    VALUES 
    (
        ?1,
        ?2,
        (SELECT id from categories WHERE name == ?3),
        ?4,
        (SELECT id from accounts WHERE name == ?5),
        (SELECT id from accounts WHERE name == ?6)
    )
"#;

pub(crate) const TRANSACTIONS_GET: &str = r#"
SELECT
    t.amount,
    t.date,
    c.name,
    c.description,
    t.currency_rate,
    acc_from.name,
    acc_from.description,
    acc_from.currency,
    acc_to.name,
    acc_to.description,
    acc_to.currency
FROM
    transactions t
LEFT JOIN
    categories c ON t.category_id = c.id
LEFT JOIN
    accounts acc_from ON t.from_id = acc_from.id
LEFT JOIN
    accounts acc_to ON t.to_id = acc_to.id
"#;
