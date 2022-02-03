use rbatis::crud::CRUD;
use rbatis::{ crud_table };
use rbatis::db::DBExecResult;
use serde:: {
    Serialize,
    Deserialize,
};
use crate::util::{date_utils, mysql};

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogFiles {
    // 主键
    pub id:                 Option<usize>,
    pub file_url:           Option<String>,
    pub upload_time:        Option<date_utils::DateTimeUtil>,
}

impl BlogFiles {
    pub async fn save(&self) -> rbatis::Result<DBExecResult> {
        let rb = mysql::this();
        rb.save(self, &[]).await
    }

    pub async fn query_all() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::this();
        rb.fetch_list().await
    }

    pub async fn query_by_id(id: usize) -> rbatis::Result<Option<Self>> {
        let rb = mysql::this();
        rb.fetch_by_column("id", &id).await
    }

    pub async fn delete(id: usize) {
        let rb = mysql::this();
        rb.remove_by_wrapper::<Self>(rb.new_wrapper().set_dml("delete").eq("id",&id)).await;
    }
}