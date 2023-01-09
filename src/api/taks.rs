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

#[get("/task/{task_global_id}")]

pub async fn get_task() -> Json<String> {
    Json("Hello world".to_string())
}