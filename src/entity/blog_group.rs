use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::{ crud_table };
use serde:: {
    Serialize,
    Deserialize,
};
use crate::util::mysql;

#[crud_table]
#[derive(Serialize, Deserialize)]
pub struct BlogGroup {
    pub id: usize,
    pub group_key: String,
    pub group_value: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl BlogGroup {
    pub async fn query_all() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::this();
        rb.fetch_list().await
    }

    pub async fn query_by_id(id: usize) -> rbatis::Result<Option<Self>>{
        let rb = mysql::this();
        rb.fetch_by_column("id", &id).await
    }
}