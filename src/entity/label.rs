use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::crud_table;
use serde:: {
    Deserialize,
    Serialize,
};

use crate::mysql;

#[crud_table]
#[derive(Serialize, Deserialize)]
pub struct BlogLabel {
    pub id: usize,
    pub label_key: String,
    pub label_value: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl BlogLabel {

    pub async fn query_by_key(key: &str) -> Option<BlogLabel> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Option<Self>> =
            rb.fetch_by_column("label_key", &key).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("[{}] 查询标签异常，异常信息为: {}", key, e);
                None
            }
        }
    }
}