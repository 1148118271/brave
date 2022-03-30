use serde:: {
    Serialize,
    Deserialize,
};

use rbatis::{ crud_table };
use rbatis::crud::CRUD;
use crate::mysql;


#[crud_table]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogConfig {
    pub id                          : Option<usize>,    //
    pub avatar_path                 : Option<usize>, // '头像路径',
    pub bg_path                     : Option<usize>, // '背景地址',
    pub blog_name                   : Option<String>, // '博客名称',
    pub blog_brief_introduction     : Option<String>, // '博客简介'
    pub is_use                      : Option<bool>    // 是否使用 1 是 0 否
}

impl BlogConfig {

    pub fn default() -> BlogConfig {
        BlogConfig {
            id: Some(0),
            avatar_path: Some(0),
            bg_path: Some(0),
            blog_name: Some("".to_string()),
            blog_brief_introduction: Some("".to_string()),
            is_use: Some(true)
        }
    }

    pub async fn query() -> Option<Self> {
        let rb = mysql::default().await;
        let result: rbatis::Result<Option<Self>> = rb.fetch_by_column("is_use", &1).await;
        match result {
            Ok(v) => v,
            Err(e) => {
                log::error!("获取博客初始信息异常, 异常信息为: {}", e);
                None
            }
        }
    }
}
