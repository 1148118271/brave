use tera::Context;
use crate::{config, mysql};
use crate::entity::{BlogConfig, BlogFiles, BlogInfo, BlogLinks};
use crate::util::html_err;

pub async fn base_info(context: &mut Context) -> bool {

    // 个人配置信息
    if !config_info(context).await { return false}
    // 热门博客
    hot_blog(context).await;
    // 归档
    archive(context).await;
    // 友链
    links(context).await;

    true
}


pub async fn links(context: &mut Context) {
    let blog_links = BlogLinks::query_all_by_flag().await;
    context.insert("links", &blog_links);
}

pub async fn archive(context: &mut Context) {
    let archive = BlogInfo::archive().await;
    context.insert("archive",&archive);
}

pub async fn hot_blog(context: &mut Context) {
    let hot_blog = BlogInfo::query_hot().await;
    context.insert("hot_blog",&hot_blog);
}

pub async fn config_info(context: &mut Context) -> bool {
    let mut result = BlogConfig::query().await;
    let result = match result {
        Ok(v) => v,
        Err(e) => {
            log::error!("获取博客初始信息异常, 异常信息为: {}", e);
            return false
        }
    };

    let blog_config = match result {
        None => BlogConfig::default(),
        Some(v) => v,
    };


    let c = config::default();
    let dn = &c.domain_name;


    let avatar_path =
        BlogFiles::query_by_id(blog_config.avatar_path.unwrap_or(0))
            .await;

    let avatar_path = match avatar_path {
        None => "static/images/lyear.png".to_string(),
        Some(v) => {
            match v.file_url {
                None => "static/images/lyear.png".to_string(),
                Some(v) => {
                    format!("{}/files/{}", &dn, v)
                }
            }
        }
    };

    context.insert("avatar_path", &avatar_path);

    if let Some(t) =
    BlogFiles::query_by_id(blog_config.favicon_path.unwrap_or(0))
        .await
    {
        if let Some(v) = t.file_url {

            let favicon_path = format!("{}/{}", dn, v);
            context.insert("favicon_path", &favicon_path);
        }
    }

    let bg_path = BlogFiles::query_by_id(blog_config.bg_path.unwrap_or(0)).await;
    let bg_path = match bg_path {
        None => "static/images/left-bg.jpg".to_string(),
        Some(v) => {
            match v.file_url {
                None => "static/images/left-bg.jpg".to_string(),
                Some(v) => {
                    format!("{}/files/{}", &dn, v)
                }
            }
        }
    };

    context.insert("bg_path", &bg_path);

    let blog_name = blog_config.blog_name.unwrap_or("陈年旧事。".to_string());
    context.insert("blog_name", &blog_name);

    let blog_brief_introduction = blog_config.blog_brief_introduction.unwrap_or("".to_string());
    context.insert("blog_brief_introduction", &blog_brief_introduction);

    true
}