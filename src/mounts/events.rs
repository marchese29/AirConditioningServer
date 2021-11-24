use crate::{
    database::{access::get_condition_for_id, ConditionDbConn},
    logic::cascade::{turn_condition_off, turn_condition_on}, errors::ACResult,
};
use diesel::Connection;

#[post("/set/condition/<condition_id>")]
pub async fn set_condition(condition_id: i32, conn: ConditionDbConn) -> ACResult<()> {
    conn.run(move |c| {
        let condition = get_condition_for_id(c, condition_id)?;
        c.transaction(|| turn_condition_on(&condition, c))
    }).await
}

#[post("/unset/condition/<condition_id>")]
pub async fn unset_condition(condition_id: i32, conn: ConditionDbConn) -> ACResult<()> {
    conn.run(move |c| {
        let condition = get_condition_for_id(c, condition_id)?;
        c.transaction(|| turn_condition_off(&condition, c))
    }).await
}
