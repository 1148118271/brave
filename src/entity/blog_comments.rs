// use serde:: {
//     Deserialize,
//     Serialize,
// };
//
// use rbatis::crud_table;
// use rbatis::crud::CRUD;
// use rbatis::db::DBExecResult;
// use crate::mysql;
//
// use crate::util::date_utils;
//
//
// #[crud_table]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct BlogComments {
//     pub id:             Option<usize>,
//     pub blog_id:        Option<usize>,
//     pub name:           Option<String>,
//     pub email:          Option<String>,
//     pub url:            Option<String>,
//     pub comment:        Option<String>,
//     pub create_time:    Option<date_utils::DateTimeUtil>
// }
//
//
// impl BlogComments {
//     pub async fn save(&self) -> rbatis::Result<DBExecResult> {
//         let rb = mysql::default().await;
//         rb.save(self, &[]).await
//     }
//     pub async fn query_by_blog_id(id: usize) -> rbatis::Result<Vec<Self>> {
//         let rb = mysql::default().await;
//         rb.fetch_list_by_column("blog_id", &[id]).await
//     }
// }