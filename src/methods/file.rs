use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::post;
use uuid::Uuid;
use crate::util::multipart_file::MultipartFile;
use crate::util::{ Results, config };

#[post("/fileUp")]
pub async fn upload(mut data: Multipart) -> Json<Results<String>> {
    let mf = MultipartFile::init(&mut data).await;

    let f = match mf {
        None => return Json(Results::error("文件解析异常,请检查上传的文件!", String::new())),
        Some(f) => f
    };

    let config = config::this();
    let up = &config.file_upload_path;

    let file_type = f.file_type();
    let fts: Vec<&str> = file_type.split("/").collect();
    let uuid = Uuid::new_v4().to_string();
    let uuid = uuid.replace("-", "");
    let file_name = format!("{}.{}", uuid, fts[1]);

    let file_uri = format!("{}{}", up, &file_name);

    f.write_all(file_uri);

    return Json(Results::success("ok", format!("/files/{}", file_name)))
}