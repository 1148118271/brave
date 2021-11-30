use actix_web::{
    HttpResponse,
    get,
    post
};
use actix_web::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use actix_web::web::{Form, Json, Path, Query};
use chrono::Local;
use log4rs::append::Append;
use tera::Context;


use crate::entity::{BlogComments, BlogDetails, BlogInfo, BlogUser, vo};
use crate::util::tera::TeraEntity;
use crate::util::{date_utils, html, html_err, Results};
use super::template;

use serde:: {
    Serialize,
    Deserialize,
};



#[get("/")]
pub async fn index(params: Query<vo::BlogPage>) -> HttpResponse {
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

    // 获取博客
    let res = BlogInfo::query_paging(params.page_num.unwrap(), params.limit_num.unwrap(), params.group_id).await;
    if res.is_err() {
        log::error!("获取博客列表异常, 异常信息为: {}", res.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let (all, page_count) = res.unwrap();

    // 获取最新的五条博客
    let new_blog = BlogInfo::query_paging(1, 5, None).await;
    if new_blog.is_err() {
        log::error!("获取博客列表异常, 异常信息为: {}", new_blog.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let (new_blog, _) = new_blog.unwrap();

    // 获取博客分类列表
    let gr = vo::GroupRes::query_blog_group().await;
    if gr.is_err() {
        log::error!("获取博客列表异常, 异常信息为: {}", gr.err().unwrap_or(rbatis::Error::E("未知异常!".to_string())));
        return html_err();
    }
    let gr = gr.unwrap();

    let mut context = Context::new();
    template::init(&mut context);
    context.insert("blogList", &all);
    context.insert("newBlogList", &new_blog);
    context.insert("newBlogList", &new_blog);
    context.insert("groupBlog", &gr);
    context.insert("groupId", &params.group_id);
    context.insert("page_count", &page_count);
    context.insert("page_num", &page_num);
    context.insert("limit_num", &limit_num);
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
    template::init(&mut context);

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

#[post("/blog/comment/save")]
pub async fn comment_save(params: Form<BlogComments>) -> Json<Results<String>> {
    let mut comments = params.0;
    comments.create_time = Some(date_utils::DateTimeUtil::from(Local::now()));
    comments.save().await;
    Json(Results::success("成功！", String::new()))
}
