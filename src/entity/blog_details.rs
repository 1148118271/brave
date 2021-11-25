use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::{ crud_table };
use serde:: {
    Serialize,
    Deserialize,
};
use crate::util::mysql;

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogDetails {
    // 主键
    id: usize,
    // 博客信息关联id
    blog_info_id: usize,
    // 博客详细信息
    details: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime
}

impl BlogDetails {
    pub async fn query_by_blog_info_id(blog_info_id: usize) -> rbatis::Result<Option<Self>>{
        let rb = mysql::this();
        rb.fetch_by_column("blog_info_id", &blog_info_id).await
    }
}