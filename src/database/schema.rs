table! {
    action_interfaces (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

table! {
    actions (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        action_interface_id -> Integer,
    }
}

table! {
    conditions (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        is_on -> Bool,
    }
}

table! {
    trigger_actions (id) {
        id -> Integer,
        trigger_id -> Integer,
        action_id -> Integer,
        is_engage_action -> Bool,
    }
}

table! {
    trigger_conditions (id) {
        id -> Integer,
        triggered_id -> Integer,
        condition_id -> Integer,
    }
}

table! {
    trigger_triggers (id) {
        id -> Integer,
        triggered_id -> Integer,
        triggering_id -> Integer,
    }
}

table! {
    triggers (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        needs_all -> Bool,
    }
}

joinable!(actions -> action_interfaces (action_interface_id));
joinable!(trigger_actions -> actions (action_id));
joinable!(trigger_actions -> triggers (trigger_id));
joinable!(trigger_conditions -> conditions (condition_id));
joinable!(trigger_conditions -> triggers (triggered_id));

allow_tables_to_appear_in_same_query!(
    action_interfaces,
    actions,
    conditions,
    trigger_actions,
    trigger_conditions,
    trigger_triggers,
    triggers,
);
