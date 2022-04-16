/// 友链
/// 2022-04-16 17:56:37








use actix_web::{get, HttpResponse};
use tera::Context;

use crate::entity::BlogLinks;
use crate::html;

/// 友链信息
#[get("/links")]
pub async fn links() -> HttpResponse {
    let mut context = Context::new();
    let vec = BlogLinks::query_all_by_flag().await;
    context.insert("links", &vec);
    html!{"links".to_string(), &context}
}