use rocket::{http::Status, serde::json::Json};

use crate::database::access::{get_actions, get_conditions};
use crate::database::ConditionDbConn;
use crate::model::request::CreateConditionRequest;
use crate::model::response::{ActionDescription, ConditionDescription};
use crate::model::{request::CreateActionRequest, response::ShallowAction};

#[post("/action", data = "<req>")]
pub async fn create_action(
    req: Json<CreateActionRequest>,
    conn: ConditionDbConn,
) -> Result<Json<ShallowAction>, Status> {
    todo!()
}

#[post("/condition", data = "<req>")]
pub async fn create_condition(
    req: Json<CreateConditionRequest>,
    conn: ConditionDbConn,
) -> Result<Json<ConditionDescription>, Status> {
    todo!()
}

#[get("/actions")]
pub async fn list_actions(conn: ConditionDbConn) -> Result<Json<Vec<ShallowAction>>, Status> {
    let response = conn.run(move |c| { get_actions(c) }).await;
    if let Ok(triggers) = response {
        Ok(Json(triggers.iter().map(ShallowAction::from_trigger).collect()))
    } else {
        Err(Status::FailedDependency)
    }
}

#[get("/conditions")]
pub async fn list_conditions(
    conn: ConditionDbConn,
) -> Result<Json<Vec<ConditionDescription>>, Status> {
    let response = conn.run(move |c| { get_conditions(c) }).await;
    if let Ok(conditions) = response {
        Ok(Json(conditions.iter().map(ConditionDescription::from_condition).collect()))
    } else {
        Err(Status::FailedDependency)
    }
}

#[get("/action/<id>")]
pub async fn describe_action(
    id: u32,
    conn: ConditionDbConn,
) -> Result<Option<Json<ActionDescription>>, Status> {
    todo!()
}
