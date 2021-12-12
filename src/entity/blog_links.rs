use serde:: {
    Serialize,
    Deserialize,
};

use rbatis::{ crud_table };
use rbatis::crud::CRUD;
use rbatis::db::DBExecResult;
use crate::util::{date_utils, mysql};


#[crud_table]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogLinks  {
    pub id:             Option<usize>,
    pub link_name:      Option<String>,
    pub link_url:       Option<String>,
    pub flag:           Option<String>,
    pub create_time:    Option<date_utils::DateTimeUtil>

}

impl BlogLinks {
    pub async fn query_all_by_flag() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::this();
        rb.fetch_list_by_column("flag", &["1".to_string()]).await
    }

    pub async fn save(&self) -> rbatis::Result<DBExecResult> {
        let rb = mysql::this();
        rb.save(self, &[]).await
    }
}