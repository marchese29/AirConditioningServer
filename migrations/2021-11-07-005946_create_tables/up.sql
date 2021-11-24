CREATE TABLE conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    is_on BOOLEAN NOT NULL
);

CREATE TABLE triggers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    needs_all BOOLEAN NOT NULL
);

CREATE TABLE action_interfaces (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    action_interface_id INTEGER NOT NULL,
    FOREIGN KEY(action_interface_id) REFERENCES action_interfaces(id)
);

CREATE TABLE trigger_conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    triggered_id INTEGER NOT NULL,
    condition_id INTEGER NOT NULL,
    FOREIGN KEY(triggered_id) REFERENCES triggers(id),
    FOREIGN KEY(condition_id) REFERENCES conditions(id)
);

CREATE TABLE trigger_triggers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    triggered_id INTEGER NOT NULL,
    triggering_id INTEGER NOT NULL,
    FOREIGN KEY(triggered_id) REFERENCES triggers(id),
    FOREIGN KEY(triggering_id) REFERENCES triggers(id)
);

CREATE TABLE trigger_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    trigger_id INTEGER NOT NULL,
    action_id INTEGER NOT NULL,
    is_engage_action BOOLEAN NOT NULL,
    FOREIGN KEY(trigger_id) REFERENCES triggers(id),
    FOREIGN KEY(action_id) REFERENCES actions(id)
);