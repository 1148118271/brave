// use rbatis::crud::CRUD;
// use rbatis::crud_table;
// use rbatis::db::DBExecResult;
// use serde:: {
//     Deserialize,
//     Serialize,
// };
// use crate::mysql;
// use crate::util::date_utils;
//
// #[crud_table]
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct BlogDetails {
//     // 主键
//     pub id:                 Option<usize>,
//     // 博客信息关联id
//     pub blog_info_id:       Option<usize>,
//     // 博客详细信息
//     pub details:            Option<String>,
//     pub create_time:        Option<date_utils::DateTimeUtil>,
//     pub update_time:        Option<date_utils::DateTimeUtil>
// }
//
// impl BlogDetails {
//     pub async fn query_by_blog_info_id(blog_info_id: usize) -> rbatis::Result<Option<Self>>{
//         let rb = mysql::default().await;
//         rb.fetch_by_column("blog_info_id", &blog_info_id).await
//     }
//
//     pub async fn save(&self) -> rbatis::Result<DBExecResult> {
//         let rb = mysql::default().await;
//         rb.save(self, &[]).await
//     }
// }