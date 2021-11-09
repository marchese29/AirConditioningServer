table! {
    conditions (id) {
        id -> Integer,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        is_on -> Bool,
        trigger_id -> Nullable<Integer>,
    }
}

table! {
    trigger_conditions (id) {
        id -> Integer,
        trigger_id -> Integer,
        condition_id -> Integer,
    }
}

table! {
    triggers (id) {
        id -> Integer,
        action_name -> Nullable<Text>,
        action_description -> Nullable<Text>,
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

joinable!(conditions -> triggers (trigger_id));
joinable!(trigger_conditions -> conditions (condition_id));
joinable!(trigger_conditions -> triggers (trigger_id));
joinable!(webhooks -> triggers (trigger_id));

allow_tables_to_appear_in_same_query!(conditions, trigger_conditions, triggers, webhooks,);
