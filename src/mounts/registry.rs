use diesel::Connection;
use rocket::{http::Status, serde::json::Json};

use crate::database::access::{create_condition, get_actions, get_conditions};
use crate::database::models::NewCondition;
use crate::database::ConditionDbConn;
use crate::logic::create::assemble_trigger;
use crate::logic::describe::traverse_trigger;
use crate::model::request::CreateConditionRequest;
use crate::model::request::CreateTriggerRequest;
use crate::model::response::{ConditionDescription, ShallowTrigger, TriggerDescription};

#[post("/action", data = "<req>")]
pub async fn create_action(
    req: Json<CreateTriggerRequest>,
    conn: ConditionDbConn,
) -> Result<Json<TriggerDescription>, Status> {
    conn.run(move |c| {
        if let Ok(description) = c.transaction(|| assemble_trigger(&req, c)) {
            Ok(Json(description))
        } else {
            Err(Status::FailedDependency)
        }
    })
    .await
}

#[post("/condition", data = "<req>")]
pub async fn create_new_condition(
    req: Json<CreateConditionRequest>,
    conn: ConditionDbConn,
) -> Result<Json<ConditionDescription>, Status> {
    conn.run(move |c| {
        let new_condition = NewCondition {
            name: req.name.clone(),
            description: req.description.clone(),
            is_on: req.is_on,
        };

        if let Ok(condition) = c.transaction(|| create_condition(c, new_condition)) {
            Ok(Json(ConditionDescription::from_condition(&condition)))
        } else {
            Err(Status::FailedDependency)
        }
    })
    .await
}

#[get("/actions")]
pub async fn list_triggers(conn: ConditionDbConn) -> Result<Json<Vec<ShallowTrigger>>, Status> {
    conn.run(move |c| {
        if let Ok(triggers) = c.transaction(|| get_actions(c)) {
            Ok(Json(
                triggers.iter().map(ShallowTrigger::from_trigger).collect(),
            ))
        } else {
            Err(Status::FailedDependency)
        }
    })
    .await
}

#[get("/conditions")]
pub async fn list_conditions(
    conn: ConditionDbConn,
) -> Result<Json<Vec<ConditionDescription>>, Status> {
    conn.run(move |c| {
        if let Ok(conditions) = c.transaction(|| get_conditions(c)) {
            Ok(Json(
                conditions
                    .iter()
                    .map(ConditionDescription::from_condition)
                    .collect(),
            ))
        } else {
            Err(Status::FailedDependency)
        }
    })
    .await
}

#[get("/action/<id>")]
pub async fn describe_action(
    id: i32,
    conn: ConditionDbConn,
) -> Result<Option<Json<TriggerDescription>>, Status> {
    conn.run(move |c| match c.transaction(|| traverse_trigger(id, c)) {
        Ok(Some(description)) => Ok(Some(Json(description))),
        Ok(None) => Ok(None),
        Err(_) => Err(Status::FailedDependency),
    })
    .await
}
