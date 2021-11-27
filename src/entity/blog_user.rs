use chrono::NaiveDateTime;
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
pub struct BlogUser {
    // 主键
    pub id: Option<u64>,
    // 账号
    pub account: Option<String>,
    // 密码
    pub password: Option<String>,
    // 名称
    pub name: Option<String>,
    // 性别 0男 1女
    pub sex: Option<u8>,
    // 年龄
    pub age: Option<u16>,
    // 手机号
    pub phone: Option<String>,
    // 邮箱
    pub email: Option<String>,
    // 头像
    pub portrait: Option<String>,
    // 创建时间
    pub create_time: Option<date_utils::DateTimeUtil>,
    // 修改时间
    pub update_time: Option<date_utils::DateTimeUtil>,
    // 逻辑删除标志 0 已删除 1 未删除
    pub del: Option<u8>,
}

impl BlogUser {
    pub async fn query_by_account(account: &str) -> rbatis::Result<Option<Self>> {
        let rb = mysql::this();
        rb.fetch_by_column("account", &account).await
    }

    pub async fn query_all() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::this();
        rb.fetch_list().await
    }

    pub async fn save(&self) -> rbatis::Result<DBExecResult> {
        let rb = mysql::this();
        rb.save(self, &[]).await
    }
}