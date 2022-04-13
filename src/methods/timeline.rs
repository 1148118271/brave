use std::collections::HashMap;

use actix_web::{
    get,
    HttpResponse
};
use actix_web::web::Query;
use tera::Context;

use crate::entity::BlogInfo;
use crate::html;

#[get("/timeline")]
pub async fn timeline(params: Query<HashMap<String, String>>) -> HttpResponse {
    // 获取base信息
    let mut context = Context::new();

    if let Some(v) = params.get("t") {
        let vec = BlogInfo::archive_by_year(v).await;
        if !vec.is_empty() {
            context.insert("blog_infos", &vec)
        }
    }

    html!{"timeline".to_string(), &context}
}