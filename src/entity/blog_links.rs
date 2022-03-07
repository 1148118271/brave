use serde:: {
    Deserialize,
    Serialize,
};

use rbatis::crud_table;
use rbatis::crud::CRUD;
use rbatis::db::DBExecResult;
use crate::mysql;
use crate::util::date_utils;


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
    pub async fn query_all_by_flag() -> Vec<Self> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Vec<Self>> = rb.fetch_list_by_column("flag", &["1".to_string()]).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("查询友链异常, 异常信息为: {}", e);
                vec![]
            }
        }
    }
}