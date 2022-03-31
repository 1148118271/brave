use tera::Context;

use crate::config;
use crate::entity::{BlogConfig, BlogFiles, BlogInfo, BlogLabel, BlogLinks};

/// 默认昵称
const DEFAULT_BLOG_NAME: &str = "陈年旧事。";
/// 默认简介
const DEFAULT_BLOG_BRIEF_INTRODUCTION: &str = "";
/// 默认头像地址
const DEFAULT_AVATAR_PATH: &str = "/static/images/blog/avatar.jpg";
/// 默认背景图片
const DEFAULT_BG_PATH: &str = "/static/images/blog/bg1.jpg";

struct BaseInfo {
    context: Context,
    time: chrono::NaiveDateTime
}

impl BaseInfo {
    async fn new() -> Self {
        let mut info = BaseInfo {
            context: Context::new(),
            time:  chrono::Local::now().naive_local()
        };
        base_info(&mut info.context).await;
        info
    }
    async fn over_one_day(&mut self) {
        let current_time = chrono::Local::now().naive_local();
        let time = self.time + chrono::Duration::days(1);
        if time < current_time {
            return;
        }
        self.time = current_time;
        base_info(&mut self.context).await
    }
}

static mut BASE_INFO: Option<BaseInfo> = None;


pub async fn get_base_context() -> Context {
    unsafe {
        if BASE_INFO.is_none() {
            BASE_INFO = Some(BaseInfo::new().await);
            return BASE_INFO.as_ref().unwrap().context.clone();
        }
        BASE_INFO.as_mut().unwrap().over_one_day().await;
        BASE_INFO.as_ref().unwrap().context.clone()
    }

}


pub async fn base_info(context: &mut Context) {
    // 个人配置信息
    config_info(context).await;
    // 热门博客
    hot_blog(context).await;
    // 归档
    archive(context).await;
    // 友链
    links(context).await;
    // 标签
    label(context).await;
}

pub async fn label(context: &mut Context) {
    let labels = BlogLabel::query_all().await;
    context.insert("labels", &labels);
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

pub async fn config_info(context: &mut Context) {
    let result = BlogConfig::query().await;

    // 头像
    context.insert("avatar_path", DEFAULT_AVATAR_PATH);
    // 背景
    context.insert("bg_path", DEFAULT_BG_PATH);
    // 昵称
    context.insert("blog_name", DEFAULT_BLOG_NAME);
    // 简介
    context.insert("blog_brief_introduction", DEFAULT_BLOG_BRIEF_INTRODUCTION);

    // 获取初始化配置
    let blog_config = match result {
        None => return,
        Some(v) => v
    };

    // 文件配置路径
    let c = config::default();
    let dn = &c.domain_name;

    // 获取头像
    let avatar_path_file = BlogFiles::query_by_id(blog_config.avatar_path.unwrap_or(0)).await;
    if let Some(v) = avatar_path_file {
        if let Some(v) = v.file_url {
            let avatar_path_url = format!("{}/files/{}", &dn, v);
            context.insert("avatar_path", &avatar_path_url);
        }
    }

    // 获取背景
    let bg_path = BlogFiles::query_by_id(blog_config.bg_path.unwrap_or(0)).await;
    if let Some(v) = bg_path {
        if let Some(v) = v.file_url {
            let bg_path_url = format!("{}/files/{}", &dn, v);
            context.insert("bg_path", &bg_path_url);
        }
    }

    // 昵称
    if let Some(v) = blog_config.blog_name {
        context.insert("blog_name", &v);
    }

    // 简介
    if let Some(v) = blog_config.blog_brief_introduction {
        context.insert("blog_brief_introduction", &v);
    }

}