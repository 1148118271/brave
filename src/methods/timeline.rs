use actix_web::{
    get,
    HttpResponse
};
use tera::Context;

use crate::entity::BlogInfo;
use crate::html;

#[get("/timeline")]
pub async fn timeline() -> HttpResponse {
    // 获取base信息
    let mut context = Context::new();
    let vec = BlogInfo::query_all().await;
    if !vec.is_empty() {
        context.insert("blog_infos", &vec)
    }
    html!{"timeline".to_string(), &context}
}