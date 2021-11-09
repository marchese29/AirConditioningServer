use crate::database::ConditionDbConn;
use rocket::http::Status;

#[post("/set/condition/<condition_id>")]
pub async fn set_condition(condition_id: u32, conn: ConditionDbConn) -> Status {
    Status::Ok
}

#[post("/unset/condition/<condition_id>")]
pub async fn unset_condition(condition_id: u32, conn: ConditionDbConn) -> Status {
    Status::Ok
}
