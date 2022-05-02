/// 关于我
/// 2022-04-16 17:16:30
use actix_web::{get, HttpResponse};
use tera::Context;

use crate::html;

/// 获取关于我页面
#[get("/about")]
pub async fn about() -> HttpResponse {
    html!{"about".to_string(), &Context::new()}
}
