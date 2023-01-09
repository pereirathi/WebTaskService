use actix_web::{
    get,
    post,
    put,
    HttpResponse,
    error::ResponseError,
    web::{Data, Json,Path},
    http::{header::ContentType,StatusCode},
};
use serde::{Deserialize, Serialize};
use derive_more::Display;

pub struct TaskIdentifier {
    task_global_id: String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>, body: Json<Struct>) -> Json<String> {
    return Json(task_identifier.into_inner().task_global_id);
}