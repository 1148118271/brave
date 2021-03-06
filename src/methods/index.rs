use std::collections::HashMap;
use std::str::FromStr;

use actix_web::{
    get,
    HttpResponse,
};
use actix_web::web::Query;
use bson::DateTime;
use serde:: {
    Deserialize,
    Serialize,
};
use tera::Context;

use crate::{cached, html};
use crate::entity::{BlogComments, BlogInfo, BlogLabel, BlogPost};
use crate::util::{date_utils, html_err};
use crate::util::date_utils::DateTimeUtil;

#[derive(Serialize, Deserialize)]
struct Result {
    id:                 usize,
    title:              String,
    publish_time:       date_utils::DateTimeUtil,
    tags:               Vec<BlogLabel>,
    comment_count:      u64,
    comment:            Vec<BlogComments>,
    read_count:         u64,
    post:               String
}

#[get("/")]
pub async fn index(page: Query<HashMap<String, String>>) -> HttpResponse {
    // 获取博客初始化的信息
    let mut context = Context::new();
    let vb = match page_method(page, &mut context).await {
        None => return html_err(),
        Some(v) => v,
    };
    let mut results = vec![];
    blog_info(vb, &mut results).await;
    context.insert("blog_infos", &results);
    html!{"index".to_string(), context}
}


async fn blog_info(bs: Vec<BlogInfo>, results: &mut Vec<Result>) {
    for v in bs {
        // 博客详情
        if v.id.is_none() {
            continue
        }
        let mut post_html = None;
        let cached = cached::default();
        let phk = format!("{}_{}",cached::POST_HTML_KEY, v.id.unwrap());
        if let Ok(v) = cached.get(&phk) {
            if let Ok(v) = String::from_utf8(v) {
                if !v.is_empty() {
                    log::debug!("获取[{}]缓存信息.", phk);
                    post_html = Some(v);
                }
            }
        }
        if post_html.is_none() {
            match BlogPost::query_by_blog_info_id(v.id.unwrap()).await {
                None => continue,
                Some(v) => {
                    post_html = Some(v.post_html.unwrap_or(String::new()));
                    log::debug!("添加[{}]缓存信息.", phk);
                    cached.set(&phk, post_html.as_ref().unwrap()).expect("添加缓存信息失败");
                }
            }
        }
        // 标签列表
        let label_key = v.label_key.unwrap_or("".to_string()).clone();
        let label_keys: Vec<&str> = label_key.split(",").collect();
        let mut lv = vec![];
        for key in label_keys {
            if let Some(v) = BlogLabel::query_by_key(key).await {
                lv.push(v)
            }
        }
        let blog_comments = BlogComments::query_by_blog_id(v.id.unwrap()).await;
        let r = Result {
            id: v.id.unwrap_or(0),
            title: v.title.unwrap_or(String::new()),
            publish_time: v.publish_time.unwrap_or(DateTimeUtil::from(DateTime::now())),
            tags: lv,
            comment_count: blog_comments.len() as u64,
            comment: blog_comments,
            read_count: v.read_count.unwrap_or(0) as u64,
            post: post_html.unwrap()
        };
        results.push(r)
    }
}

async fn page_method(page: Query<HashMap<String, String>>, context: &mut Context) -> Option<Vec<BlogInfo>> {
    let ps = String::from("1");
    let page = page.get("p").unwrap_or(&ps);
    let mut page = u64::from_str(page).unwrap_or(1);
    if page < 1 {
        page = 1
    }
    let option = BlogInfo::query_page(page).await;
    let mut page_info = match option {
        None => return None,
        Some(v) => v
    };
    if page > page_info.pages {
        page = page_info.pages;
        match BlogInfo::query_page(page).await {
            None => return None,
            Some(v) => page_info = v
        }
    }
    // 所有页数
    context.insert("pages", &page_info.pages);
    // 当前是第几页
    context.insert("page_no", &page_info.page_no);
    page_num(page_info.pages, page_info.page_no, 10, context);
    Some(page_info.records)
}

fn page_num(pages: u64, mut page_on: u64, page: u64, context: &mut Context) {
    let mut v = vec![];
    if pages <= page {
        for i in 1..pages + 1 {
            v.push(i);
        }
    } else {
        for i in 1..pages + 1 {
            if i > 5 { break }
            if page_on % page == 0 {
                page_on = page_on - 1
            }
            let a = page_on / page;
            v.push(a + i);
        }
    }
    context.insert("page_num", &v);
}
