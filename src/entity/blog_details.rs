use rbatis::crud::CRUD;
use rbatis::crud_table;
use serde:: {
    Deserialize,
    Serialize,
};

use crate::mysql;
use crate::util::date_utils;

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogDetails {
    // 主键
    pub id:                 Option<usize>,
    // 博客信息关联id
    pub blog_info_id:       Option<usize>,
    // 博客详细信息
    pub details:            Option<String>,
    pub create_time:        Option<date_utils::DateTimeUtil>,
    pub update_time:        Option<date_utils::DateTimeUtil>
}

impl BlogDetails {
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