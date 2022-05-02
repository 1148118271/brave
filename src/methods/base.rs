use actix_web::get;
use actix_web::web::Json;
use tera::Value;

use crate::{cached, config};
use crate::entity::{BlogConfig, Files};

/// 默认昵称
const DEFAULT_BLOG_NAME: &str = "陈年旧事。";
/// 默认简介
const DEFAULT_BLOG_BRIEF_INTRODUCTION: &str = "";
/// 默认头像地址
const DEFAULT_AVATAR_PATH: &str = "/static/images/blog/avatar.jpg";
/// 默认背景图片
const DEFAULT_BG_PATH: &str = "/static/images/blog/bg1.jpg";


/// 获取初始化配置信息
#[get("/base/conf")]
pub async fn base_conf() -> Json<Value> {
    let cached = cached::default();
    if let Ok(result) = cached.get(cached::CONF_KEY) {
        if result.len() > 0 {
            if let Ok(vs) = String::from_utf8(result) {
                if let Ok(v) = vs.parse() {
                    log::debug!("获取[{}]缓存信息.", cached::CONF_KEY);
                    return Json(v)
                }
            }
        }
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
    let vs = value.to_string();
    log::debug!("添加[{}]缓存信息.", cached::CONF_KEY);
    cached.set(cached::CONF_KEY, &vs).expect("添加缓存信息失败");
    Json(value)
}