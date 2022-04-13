use rbatis::crud::CRUD;
use serde:: {
    Deserialize,
    Serialize,
};

use crate::mysql;
use crate::util::date_utils;

/// 博客帖子详情
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BlogPost {
    /// 主键
    pub id: Option<u64>,
    /// 博客信息id
    pub blog_info_id: Option<u64>,
    /// 文本
    pub post_text: Option<String>,
    /// html
    pub post_html: Option<String>,
    /// 创建时间
    pub create_time: Option<date_utils::DateTimeUtil>,
    /// 修改时间
    pub update_time: Option<date_utils::DateTimeUtil>,
}
impl BlogPost {
    pub async fn query_by_blog_info_id(blog_info_id: usize) -> Option<Self> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Option<Self>> = rb.fetch_by_column("blog_info_id", &blog_info_id).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询博客详情异常, 异常信息为: {}", e);
                None
            }
        }
    }
}