use std::collections::HashMap;
use actix_web::{
    HttpResponse,
    get,
    post
};
use actix_web::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use actix_web::web::{Form, Json, Path, Query};
use chrono::Local;
use serde_json::Value;
use tera::Context;


use crate::entity::{BlogComments, BlogDetails, BlogInfo, BlogUser, vo};
use crate::util::tera::TeraEntity;
use crate::util::{date_utils, html, html_err, Results};


#[get("/")]
pub async fn index(pn: Query<HashMap<String, String>>) -> HttpResponse {

    let mut page_num = 1_u64;
    let limit_num = 5_u64;

    let pns = (*pn).get("page_num");
    match pns {
        Some(vo) => page_num = vo.parse::<u64>().unwrap_or(1),
        _ => {}
    };

    // 获取博客
    let res = BlogInfo::query_paging(page_num, limit_num).await;
    let (all, page_count) = match res {
        Ok(v) => v,
        Err(e) => {
            log::error!("获取博客列表异常, 异常信息为: {}", e);
            return html_err();
        }
    };

    let mut context = Context::new();
    context.insert("blogList", &all);
    context.insert("page_count", &page_count);
    context.insert("page_num", &page_num);
    let string = TeraEntity::render("view/index", &context).unwrap();
    html(string)
}


#[get("/blog/details/{id}")]
pub async fn details(id: Path<usize>) -> HttpResponse {
    let result = BlogInfo::add_read(id.0).await;
    if result.is_err() {
        return html_err();
    }

    let result = BlogDetails::query_by_blog_info_id(id.0).await;
    if result.is_err() {
        log::error!("获取博客详情异常, 异常信息为: {}", result.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let result = result.unwrap();

    let mut context = Context::new();

    match result {
        None => context.insert("details", "<h1>暂无详情</h1>"),
        Some(v) => context.insert("details", &v.details)
    }

    let blog_info = BlogInfo::query_by_id(id.0).await;
    if blog_info.is_err() {
        log::error!("获取博客信息异常, 异常信息为: {}", blog_info.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let blog_info = blog_info.unwrap();
    if blog_info.is_none() {
        log::error!("获取博客信息异常");
        return html_err();
    }

    let mut blog_info = blog_info.unwrap();
    blog_info.user_name = Some("子木".to_string());

    context.insert("blogInfo", &blog_info);

    let string = TeraEntity::render("view/details", &context).unwrap();
    html(string)
}

#[get("/blog/comment/{id}")]
pub async fn comment(id: Path<usize>) -> Json<Results<Vec<BlogComments>>> {
    let result = BlogComments::query_by_blog_id(id.into_inner()).await;
    let mut rv: Vec<BlogComments> = match result.ok() {
        None => {
            log::error!("获取博客评论列表异常！");
            vec![]
        }
        Some(v) => v
    };
    Json(Results::success("成功！", rv))
}


#[get("/blog/group")]
pub async fn group() -> HttpResponse {
    let mut context = Context::new();
    let group = vo::GroupRes::query_blog_group().await;
    let group = match group {
        Ok(v) => v,
        Err(e) => {
            log::error!("获取分组列表异常, 异常信息为: {}", e);
            vec![]
        }
    };
    context.insert("group", &group);
    let string = TeraEntity::render("view/group", &context).unwrap();
    html(string)
}

#[post("/blog/comment/save")]
pub async fn comment_save(params: Form<BlogComments>) -> Json<Results<String>> {
    let mut comments = params.0;
    comments.create_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    comments.save().await;
    Json(Results::success("成功！", String::new()))
}
