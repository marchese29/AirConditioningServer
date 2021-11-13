use super::schema::*;
use diesel::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "triggers"]
pub struct Trigger {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub needs_all: bool,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "triggers"]
pub struct NewTrigger {
    pub name: String,
    pub description: String,
    pub needs_all: bool,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "conditions"]
pub struct Condition {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_on: bool,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "conditions"]
pub struct NewCondition {
    pub name: String,
    pub description: String,
    pub is_on: bool,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger, foreign_key = "triggered_id")]
#[belongs_to(Condition)]
#[table_name = "trigger_conditions"]
pub struct TriggerCondition {
    pub triggered_id: i32,
    pub condition_id: i32,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger, foreign_key = "triggered_id")]
#[table_name = "trigger_triggers"]
pub struct TriggerTrigger {
    pub triggered_id: i32,
    pub triggering_id: i32,
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Trigger)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub engage_url: String,
    pub disengage_url: Option<String>,
    pub trigger_id: i32,
}
