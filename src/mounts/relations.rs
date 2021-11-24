use diesel::Connection;
use rocket::serde::json::Json;

use crate::database::access::{create_condition, get_conditions, get_triggers};
use crate::database::models::NewCondition;
use crate::database::ConditionDbConn;
use crate::errors::ACApiResult;
use crate::logic::create::assemble_trigger;
use crate::logic::describe::traverse_trigger;
use crate::model::request::CreateConditionRequest;
use crate::model::request::CreateTriggerRequest;
use crate::model::response::{ConditionDescription, ShallowTrigger, TriggerDescription};

#[post("/trigger", data = "<req>")]
pub async fn create_trigger(
    req: Json<CreateTriggerRequest>,
    conn: ConditionDbConn,
) -> ACApiResult<TriggerDescription> {
    Ok(Json(
        conn.run(move |c| c.transaction(|| assemble_trigger(&req, c)))
            .await?,
    ))
}

#[post("/condition", data = "<req>")]
pub async fn create_new_condition(
    req: Json<CreateConditionRequest>,
    conn: ConditionDbConn,
) -> ACApiResult<ConditionDescription> {
    let condition = conn
        .run(move |c| {
            let new_condition = NewCondition {
                name: req.name.clone(),
                description: req.description.clone(),
                is_on: req.is_on,
            };

            c.transaction(|| create_condition(c, new_condition))
        })
        .await?;

    Ok(Json(ConditionDescription::from_condition(&condition)))
}

#[get("/triggers")]
pub async fn list_triggers(conn: ConditionDbConn) -> ACApiResult<Vec<ShallowTrigger>> {
    conn.run(move |c| {
        let triggers = get_triggers(c)?;
        Ok(Json(
            triggers.iter().map(ShallowTrigger::from_trigger).collect(),
        ))
    })
    .await
}

#[get("/conditions")]
pub async fn list_conditions(conn: ConditionDbConn) -> ACApiResult<Vec<ConditionDescription>> {
    conn.run(move |c| {
        let conditions = get_conditions(c)?;
        Ok(Json(
            conditions
                .iter()
                .map(ConditionDescription::from_condition)
                .collect(),
        ))
    })
    .await
}

#[get("/trigger/<id>")]
pub async fn describe_trigger(id: i32, conn: ConditionDbConn) -> ACApiResult<TriggerDescription> {
    Ok(Json(conn.run(move |c| traverse_trigger(id, c)).await?))
}
