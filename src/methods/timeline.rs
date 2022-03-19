use std::collections::HashMap;
use actix_web::{
    HttpResponse,
    get
};
use actix_web::web::Query;
use tera::Context;
use crate::entity::BlogInfo;
use crate::methods::base;
use crate::util::html_err;
use crate::html;

#[get("/timeline")]
pub async fn timeline(params: Query<HashMap<String, String>>) -> HttpResponse {

    let mut context = Context::new();
    // 获取base信息
    if !base::base_info(&mut context).await {
        return html_err()
    }
    if let Some(v) = params.get("t") {
        let vec = BlogInfo::archive_by_year(v).await;
        if !vec.is_empty() {
            context.insert("blog_infos", &vec)
        }
    }

    html!{"view/timeline".to_string(), &context}
}