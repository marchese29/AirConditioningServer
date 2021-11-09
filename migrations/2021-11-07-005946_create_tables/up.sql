CREATE TABLE conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR,
    description VARCHAR,
    is_on BOOLEAN NOT NULL,
    trigger_id INTEGER,
    FOREIGN KEY(trigger_id) REFERENCES triggers(id)
);

CREATE TABLE webhooks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    engage_url VARCHAR NOT NULL,
    disengage_url VARCHAR,
    trigger_id INTEGER NOT NULL,
    FOREIGN KEY(trigger_id) REFERENCES triggers(id)
);

CREATE TABLE triggers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    action_name VARCHAR,
    action_description VARCHAR
);

CREATE TABLE trigger_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    trigger_id INTEGER NOT NULL,
    condition_id INTEGER NOT NULL,
    FOREIGN KEY(trigger_id) REFERENCES triggers(id),
    FOREIGN KEY(condition_id) REFERENCES conditions(id)
);