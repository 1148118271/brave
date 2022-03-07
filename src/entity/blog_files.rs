use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::db::DBExecResult;
use serde:: {
    Deserialize,
    Serialize,
};
use crate::mysql;
use crate::util::date_utils;

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
        let rb = mysql::default().await;
        rb.save(self, &[]).await
    }

    pub async fn query_all() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::default().await;
        rb.fetch_list().await
    }

    pub async fn query_by_id(id: usize) -> Option<Self> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Option<Self>> = rb.fetch_by_column("id", &id).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("根据id查询文件异常, 异常信息为: {}", e);
                None
            }
        }
    }

    pub async fn delete(id: usize) {
        let rb = mysql::default().await;
        rb.remove_by_wrapper::<Self>(rb.new_wrapper().set_dml("delete").eq("id",&id)).await;
    }
}