use bson::{Bson, DateTime};
use chrono::{Local, NaiveDateTime};
use rbatis::crud::{CRUD, CRUDTable};
use rbatis::{crud_table, py_sql, DateTimeNative, Page, PageRequest, Error};
use rbatis::db::DBExecResult;
use rbatis::executor::{Executor, RbatisExecutor};
use serde:: {
    Serialize,
    Deserialize,
};
use crate::entity::{ BlogGroup, BlogUser};
use crate::util::{ mysql, date_utils };

#[crud_table(table_columns:"id, title, generalize, user_account, publish_time, group_id, read_count, is_publish, create_time, update_time")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogInfo {
    // 主键
    pub id: Option<usize>,
    // 标题
    pub title: Option<String>,
    // 简介/概括
    pub generalize: Option<String>,
    // 用户账号
    pub user_account: Option<String>,
    // 用户名称
    pub user_name: Option<String>,
    // 发布时间
    pub publish_time: Option<date_utils::DateTimeUtil>,
    // 分组关联id
    pub group_id: Option<usize>,
    // 分组名称
    pub group_name: Option<String>,
    // 评论次数
    pub comment_count: Option<u32>,
    // 阅读次数
    pub read_count: Option<u32>,
    // 是否发布 0 未发布, 1 已发布
    pub is_publish: Option<String>,
    pub create_time: Option<date_utils::DateTimeUtil>,
    pub update_time: Option<date_utils::DateTimeUtil>
}


impl BlogInfo {
    pub async fn query_paging(page_num: isize, limit_num: isize, params: Option<isize>) -> rbatis::Result<(Vec<Self>, u64)> {
        let rb = mysql::this();
        let wrapper = match params {
            None => {rb.new_wrapper().eq("is_publish", 1).order_by(false, &["create_time"])}
            Some(v) => {rb.new_wrapper().eq("is_publish", 1).eq("group_id", v).order_by(false, &["create_time"])}
        };
        let pr = PageRequest::new(page_num as u64, limit_num as u64);
        let page: Page<Self> = rb.fetch_page_by_wrapper(wrapper, &pr).await?;

        let pages = page.pages;

        let mut result = page.records;
        if result.len() == 0 {
           return Ok((result, 1u64))
        }

        for mut x in &mut result {
            let acc = match &x.user_account {
                None => "gxk",
                Some(x) => x.as_str()
            };
            let account = BlogUser::query_by_account(acc).await?;
            let user_name = match account {
                None => "子木".to_string(),
                Some(u) => u.name.unwrap_or("子木".to_string())
            };
            x.user_name = Some(user_name);

            let blog_group = BlogGroup::query_by_id(x.group_id.unwrap_or(0)).await?;
            let group_name = match blog_group {
                None => "默认分组".to_string(),
                Some(v) => v.group_value
            };

            x.group_name = Some(group_name);

            x.comment_count = Some(0);

        }

        Ok((result, pages))
    }

    pub async fn query_all() -> rbatis::Result<Vec<Self>> {
        let rb = mysql::this();
        let wrapper = rb.new_wrapper().order_by(false, &["create_time"]);
        let mut result: Vec<BlogInfo> = rb.fetch_list_by_wrapper(wrapper).await?;
        if result.len() == 0 {
            return Ok(result)
        }
        for mut x in &mut result {
            let acc = match &x.user_account {
                None => "gxk",
                Some(x) => x.as_str()
            };
            let account = BlogUser::query_by_account(acc).await?;
            let user_name = match account {
                None => "子木".to_string(),
                Some(u) => u.name.unwrap_or("子木".to_string())
            };
            x.user_name = Some(user_name);

            let blog_group = BlogGroup::query_by_id(x.group_id.unwrap_or(0)).await?;
            let group_name = match blog_group {
                None => "默认分组".to_string(),
                Some(v) => v.group_value
            };

            x.group_name = Some(group_name);

            x.comment_count = Some(0);

        }
        Ok(result)
    }

    pub async fn save(&self) -> rbatis::Result<DBExecResult> {
        let rb = mysql::this();
        rb.save(
            &BlogInfo1 {
                id: self.id,
                title: self.title.clone(),
                generalize: self.generalize.clone(),
                user_account: self.user_account.clone(),
                publish_time: self.publish_time,
                group_id: self.group_id,
                read_count: self.read_count,
                is_publish: self.is_publish.clone(),
                create_time: self.create_time,
                update_time: self.update_time
            },
            &[],
        ).await

    }

    pub async fn delete(id: String) {
        let rb = mysql::this();
        rb.remove_by_wrapper::<Self>(rb.new_wrapper().set_dml("delete").eq("id",&id)).await;
    }


    pub async fn published(id: usize) {
        let rb = mysql::this();
        let wrapper = rb.new_wrapper().eq("id", id);
        rb.update_by_wrapper::<PublishedVo>(&PublishedVo { publish_time: Some(date_utils::DateTimeUtil::from(Local::now())), is_publish: Some("1".to_string()) }, wrapper, &[]).await;
    }

    pub async fn add_read(id: usize) -> rbatis::Result<()> {
        let rb = mysql::this();
        let option = Self::query_by_id(id).await?;
        if option.is_none() {
            log::error!("未查到博客信息, 查询条件为: [id = {}]", id);
            return Err(rbatis::Error::E("博客信息查询为空!".to_string()))
        }

        let mut info = option.unwrap();
        let i = info.read_count.unwrap_or(0);
        let rcv = ReadCountVo {
            read_count: Some(i + 1)
        };

        let wrapper = rb.new_wrapper().eq("id", id);
        rb.update_by_wrapper::<ReadCountVo>(&rcv, wrapper, &[]).await?;
        Ok(())
    }

    pub async fn query_by_id(id: usize) -> rbatis::Result<Option<Self>> {
        let rb = mysql::this();
        rb.fetch_by_column("id", &id).await
    }
}


#[crud_table(table_name:"blog_info")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ReadCountVo {
    // 阅读次数
    pub read_count: Option<u32>,
}


#[crud_table(table_name:"blog_info")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PublishedVo {
    pub publish_time: Option<date_utils::DateTimeUtil>,
    pub is_publish: Option<String>,
}

#[crud_table(table_name:"blog_info")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct BlogInfo1{
    // 主键
    pub id: Option<usize>,
    // 标题
    pub title: Option<String>,
    // 简介/概括
    pub generalize: Option<String>,
    // 用户账号
    pub user_account: Option<String>,
    // 发布时间
    pub publish_time: Option<date_utils::DateTimeUtil>,
    // 分组关联id
    pub group_id: Option<usize>,
    // 阅读次数
    pub read_count: Option<u32>,
    // 是否发布 0 未发布, 1 已发布
    pub is_publish: Option<String>,
    pub create_time: Option<date_utils::DateTimeUtil>,
    pub update_time: Option<date_utils::DateTimeUtil>
}