use serde:: {
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize)]
pub struct PageParams {
    #[serde(rename = "pageSize")]
    pub page_size: Option<isize>,
    #[serde(rename = "pageNumber")]
    pub page_number: Option<isize>
}
