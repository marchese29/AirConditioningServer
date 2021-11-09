use rocket::{http::Status, serde::json::Json};

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
    todo!()
}

#[get("/conditions")]
pub async fn list_conditions(
    conn: ConditionDbConn,
) -> Result<Json<Vec<ConditionDescription>>, Status> {
    todo!()
}

#[get("/action/<id>")]
pub async fn describe_action(
    id: u32,
    conn: ConditionDbConn,
) -> Result<Option<Json<ActionDescription>>, Status> {
    todo!()
}
