use super::schema::*;
use diesel::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "triggers"]
pub struct Trigger {
    pub id: i32,
    pub action_name: Option<String>,
    pub action_description: Option<String>,
    pub needs_all: bool,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger)]
#[table_name = "conditions"]
pub struct Condition {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_on: bool,
    pub trigger_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger)]
#[belongs_to(Condition)]
#[table_name = "trigger_conditions"]
pub struct TriggerCondition {
    pub id: i32,
    pub trigger_id: i32,
    pub condition_id: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub id: i32,
    pub engage_url: String,
    pub disengage_url: Option<String>,
    pub trigger_id: i32,
}
