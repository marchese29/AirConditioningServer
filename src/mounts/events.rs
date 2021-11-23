use crate::{
    database::{access::get_condition_for_id, ConditionDbConn},
    logic::cascade::{turn_condition_off, turn_condition_on},
};
use diesel::Connection;
use rocket::http::Status;

#[post("/set/condition/<condition_id>")]
pub async fn set_condition(condition_id: i32, conn: ConditionDbConn) -> Status {
    conn.run(move |c| match get_condition_for_id(c, condition_id) {
        Ok(Some(condition)) => match c.transaction(|| turn_condition_on(&condition, c)) {
            Ok(()) => return Status::Ok,
            Err(_) => return Status::FailedDependency,
        },
        Ok(None) => return Status::NotFound,
        Err(_) => return Status::FailedDependency,
    })
    .await
}

#[post("/unset/condition/<condition_id>")]
pub async fn unset_condition(condition_id: i32, conn: ConditionDbConn) -> Status {
    conn.run(move |c| match get_condition_for_id(c, condition_id) {
        Ok(Some(condition)) => match c.transaction(|| turn_condition_off(&condition, c)) {
            Ok(()) => return Status::Ok,
            Err(_) => return Status::FailedDependency,
        },
        Ok(None) => return Status::NotFound,
        Err(_) => return Status::FailedDependency,
    })
    .await
}
