use actix_web::get;
use actix_web::web::Json;
use tera::Value;

use crate::config;
use crate::entity::{BlogConfig, Files};

/// 默认昵称
const DEFAULT_BLOG_NAME: &str = "陈年旧事。";
/// 默认简介
const DEFAULT_BLOG_BRIEF_INTRODUCTION: &str = "";
/// 默认头像地址
const DEFAULT_AVATAR_PATH: &str = "/static/images/blog/avatar.jpg";
/// 默认背景图片
const DEFAULT_BG_PATH: &str = "/static/images/blog/bg1.jpg";

/// 用户配置缓存天数
const BASE_CONF_CACHE_DAY: i64 = 2;


static mut BASE_CONF_CACHE: Option<BaseConfCache> = None;


/// 初始化配置缓存
struct BaseConfCache {
    time: chrono::NaiveDateTime,
    data: Value
}

impl BaseConfCache {
    fn init(v: Value) {
        unsafe {
            let v = Self {
                time: chrono::Local::now().naive_local(),
                data: v
            };
            BASE_CONF_CACHE = Some(v)
        }
    }

    fn is_expired() -> Option<Value> {
        unsafe {
            if BASE_CONF_CACHE.is_none() {
                return None
            }
            let b = BASE_CONF_CACHE.as_ref().unwrap();
            let current_time = chrono::Local::now().naive_local();
            let new_time = b.time + chrono::Duration::days(BASE_CONF_CACHE_DAY);
            if current_time >= new_time {
                log::info!("缓存时间为: {}, 当前时间为: {}, 到期时间为: {}, 缓存已到期!", b.time, current_time, new_time);
                return None
            }
            log::info!("缓存时间为: {}, 当前时间为: {}, 到期时间为: {}, 缓存已到期!", b.time, current_time, new_time);
            Some((&b.data).clone())
        }
    }
}



/// 获取初始化配置信息
#[get("/base/conf")]
pub async fn base_conf() -> Json<Value> {

    if let Some(v) = BaseConfCache::is_expired() {
        return Json(v)
    }

    let result = BlogConfig::query().await;
    let mut value = Value::default();
    value["avatar_path"] = Value::String(DEFAULT_AVATAR_PATH.to_string());
    value["bg_path"] = Value::String(DEFAULT_BG_PATH.to_string());
    value["blog_name"] = Value::String(DEFAULT_BLOG_NAME.to_string());
    value["blog_brief_introduction"] = Value::String(DEFAULT_BLOG_BRIEF_INTRODUCTION.to_string());

    // 获取初始化配置
    let blog_config = match result {
        None => return Json(value),
        Some(v) => v
    };

    // 文件配置路径
    let c = config::default();
    let dn = &c.domain_name;

    // 获取头像
    if let Some(id) = blog_config.avatar_path {
        let avatar_path_file = Files::query_by_id(id).await;
        if let Some(v) = avatar_path_file {
            if let Some(v) = v.file_url {
                let avatar_path_url = format!("{}/files/{}", &dn, v);
                value["avatar_path"] = Value::String(avatar_path_url);
            }
        }
    }

    // 获取背景
    if let Some(id) = blog_config.bg_path {
        let bg_path = Files::query_by_id(id).await;
        if let Some(v) = bg_path {
            if let Some(v) = v.file_url {
                let bg_path_url = format!("{}/files/{}", &dn, v);
                value["bg_path"] = Value::String(bg_path_url);
            }
        }
    }

    // 昵称
    if let Some(v) = blog_config.blog_name {
        value["blog_name"] = Value::String(v);
    }

    // 简介
    if let Some(v) = blog_config.blog_brief_introduction {
        value["blog_brief_introduction"] = Value::String(v);
    }
    BaseConfCache::init(value.clone());
    Json(value)
}