use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use actix_web::{
    get,
    HttpResponse, post
};
use actix_web::web::{Form, Json, Query};
use tera::Context;

use crate::entity::{BlogComments, BlogInfo, BlogLabel, BlogPost};
use crate::html;
use crate::methods::base;
use crate::util::{html_err, Results};

#[post("blog/comment")]
pub async fn submit_comments(params: Form<BlogComments>) -> Json<Results<String>> {
    let comment = match &params.comment {
        None => false,
        Some(v) => !v.is_empty()
    };
    if !comment {
        return Json(Results::error("评论信息不能为空！", String::new()))
    }
    let name = match &params.name {
        None => false,
        Some(v) => !v.is_empty()
    };
    if !name {
        return Json(Results::error("昵称不能为空！", String::new()))
    }

    match params.save().await {
        Err(e) => {
            log::error!("评论信息保存失败, 错误信息为: {}", e);
            return Json(Results::error("评论信息保存失败！", String::new()))
        }
        _ => {}
    }

    Json(Results::success("成功！", String::new()))

}

#[get("/post")]
pub async fn details(params: Query<HashMap<String, usize>>) -> HttpResponse {
    let context = base::get_base_context().await;

    let rc = Rc::new(RefCell::new(context));

    // 获取博客id
    let blog_id = params.get("v");
    if blog_id.is_none() {
        log::error!("获取博客详情时博客id为空!");
        return html_err()
    }
    let blog_id = *(blog_id.unwrap());

    // 阅读次数加1
    BlogInfo::add_read_count(blog_id).await;

    // 标题, 时间, 查看次数
    let (flag, tags) = title_and_publish_time(blog_id, rc.clone()).await;
    if !flag {
        return html_err()
    }
    // 标签
    get_tags(tags, rc.clone()).await;
    // 评论
    get_comments(blog_id, rc.clone()).await;
    // 博客详情
    get_details(blog_id, rc.clone()).await;

    html!{"post".to_string(), &*rc.borrow()}
}

/// 获取博客详情
async fn get_details(id: usize, c: Rc<RefCell<Context>>) {
    let mut c = c.borrow_mut();
    let bd = BlogPost::query_by_blog_info_id(id).await;
    let d = match bd {
        None => "<h1>暂无博客详情信息</h1>".to_string(),
        Some(v) => v.post_html.unwrap_or("<h1>暂无博客详情信息</h1>".to_string())
    };
    c.insert("details", &d)
}

/// 获取评论信息
async fn get_comments(id: usize, c: Rc<RefCell<Context>>) {
    let mut c = c.borrow_mut();
    let blog_comments = BlogComments::query_by_blog_id(id).await;
    c.insert("comments", &blog_comments);
    c.insert("comments_len", &blog_comments.len())
}

/// 标签
async fn get_tags(tags: Vec<String>, c: Rc<RefCell<Context>>) {
    let mut c = c.borrow_mut();
    let mut r = vec![];
    for key in tags {
        if let Some(v) = BlogLabel::query_by_key(&key).await {
            r.push(v.label_value)
        }
    }
    if r.len() <= 0 { r.push("暂无标签".to_string()) }
    c.insert("tags", &r)
}

/// 标题, 时间, 阅读
async fn title_and_publish_time(id: usize, c: Rc<RefCell<Context>>) -> (bool, Vec<String>) {
    let mut c = c.borrow_mut();
    let mut tags = vec![];
    if let Some(v) = BlogInfo::query_by_id(id).await {
        // 标题
        c.insert("title", &v.title);
        // 时间
        c.insert("publish_time", &v.publish_time);
        // 查看次数
        c.insert("read_count", &v.read_count);
        c.insert("blog_id", &id);
        // 获取所有的标签key
        if let Some(tag_str) = v.label_key {
            tags = tag_str.split(",").map(|v| String::from(v)).collect();
            return (true, tags)
        }
    }
    (false, tags)
}