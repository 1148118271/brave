use rbatis::crud::CRUD;
use serde:: {
    Deserialize,
    Serialize,
};

use crate::mysql;
use crate::util::date_utils;

/// 文件信息
#[rbatis::crud_table]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Files {
    /// 主键
    pub id: Option<u64>,
    /// 文件路径
    pub file_url: Option<String>,
    /// 文件原始名称
    pub file_original_name: Option<String>,
    /// uuid后的名称
    pub file_uuid_name: Option<String>,
    /// 文件大小
    pub file_size: Option<u64>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 上传时间
    pub upload_time: Option<date_utils::DateTimeUtil>
}

impl Files {

    /// 根据id查询
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

}