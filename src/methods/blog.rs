use actix_web::{
    HttpResponse,
    get,
};
use actix_web::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use actix_web::web::{Json, Query};
use log4rs::append::Append;
use tera::Context;


use crate::entity::{BlogInfo, BlogUser};
use crate::util::tera::TeraEntity;
use crate::util::{html, html_err, Results};
use super::template;

use serde:: {
    Serialize,
    Deserialize,
};


#[derive(Serialize, Deserialize)]
struct BlogPage {
    page_num: Option<isize>,
    limit_num: Option<isize>,
    page_count: Option<isize>,
}


#[get("/")]
pub async fn index(params: Query<BlogPage>) -> HttpResponse {
    let mut params = params.0;
    if params.page_num.is_none() {
        params.page_num = Some(1)
    }
    if params.limit_num.is_none() {
        params.limit_num = Some(4)
    }

    let page_num = params.page_num.unwrap();
    let limit_num = params.limit_num.unwrap();

    if params.page_count.is_some() {
        if page_num < 1 || page_num > params.page_count.unwrap() {
            let mut response = HttpResponse::new(StatusCode::FOUND);
            let header: &mut HeaderMap = response.headers_mut();
            let k = HeaderName::from_bytes(b"location").unwrap();
            let v = HeaderValue::from_bytes(b"/").unwrap();
            header.append(k, v);
            return response
        }
    }

    let res = BlogInfo::query_paging(params.page_num.unwrap(), params.limit_num.unwrap()).await;
    if res.is_err() {
        log::error!("获取博客列表异常, 异常信息为: {}", res.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let (all, page_count) = res.unwrap();
    let mut context = Context::new();
    template::init(&mut context);
    context.insert("blogList", &all);
    context.insert("page_count", &page_count);
    context.insert("page_num", &page_num);
    context.insert("limit_num", &limit_num);
    let string = TeraEntity::render("view/index", &context).unwrap();
    html(string)
}


#[get("/blog/details")]
pub async fn details() -> HttpResponse {
    let mut context = Context::new();
    template::init(&mut context);
    let string = TeraEntity::render("view/details", &context).unwrap();
    html(string)
}