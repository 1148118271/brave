use std::collections::HashMap;
use actix_web::{
    HttpResponse,
    get
};
use actix_web::web::Query;
use crate::entity::BlogInfo;
use crate::methods::base;
use crate::html;

#[get("blog/timeline")]
pub async fn timeline(params: Query<HashMap<String, String>>) -> HttpResponse {
    // 获取base信息
    let mut context = base::get_base_context().await;

    if let Some(v) = params.get("t") {
        let vec = BlogInfo::archive_by_year(v).await;
        if !vec.is_empty() {
            context.insert("blog_infos", &vec)
        }
    }

    html!{"blog/timeline".to_string(), &context}
}