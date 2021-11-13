table! {
    conditions (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        is_on -> Bool,
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

table! {
    webhooks (id) {
        id -> Integer,
        engage_url -> Text,
        disengage_url -> Nullable<Text>,
        trigger_id -> Integer,
    }
}

joinable!(trigger_conditions -> conditions (condition_id));
joinable!(trigger_conditions -> triggers (triggered_id));
joinable!(webhooks -> triggers (trigger_id));

allow_tables_to_appear_in_same_query!(
    conditions,
    trigger_conditions,
    trigger_triggers,
    triggers,
    webhooks,
);
