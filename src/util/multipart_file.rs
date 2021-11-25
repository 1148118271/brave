use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use actix_multipart::Multipart;
use actix_web::web::BytesMut;
use futures::StreamExt;

#[derive(Debug)]
pub struct MultipartFile {
    file_stream: Vec<u8>,
    file_name: String,
    file_type: String,
    file_size: usize
}

impl MultipartFile {

    pub async fn init(data: &mut Multipart) -> Option<MultipartFile>{
        let mfs = match Self::init_vec(data).await {
            None => None,
            Some(mut v) => {
                Some(v.swap_remove(0))
            },
        };
        mfs
    }

    pub async fn init_vec(data: &mut Multipart) -> Option<Vec<MultipartFile>> {
        let mut mfs = vec![];
        'w: loop {
            match data.next().await {
                None => break 'w,
                Some(v) => {
                    let mut field = v.unwrap();
                    let mut bytes = BytesMut::new();
                    'n: loop {
                        match field.next().await {
                            None => break 'n,
                            Some(nv) => {
                                let bt = &nv.unwrap();
                                bytes.extend_from_slice(bt)
                            }
                        }
                    }

                    let file_name = field
                        .content_disposition()
                        .unwrap();
                    let file_name = file_name.get_filename().unwrap();

                    mfs.push(
                        MultipartFile {
                                file_stream: bytes.to_vec(),
                                file_name: file_name.to_string(),
                                file_type: field.content_type().to_string(),
                                file_size: bytes.len()
                        })
                }
            }
        }
        if mfs.len() <= 0 {
            return None
        }

        Some(mfs)
    }

    pub fn file_type(&self) -> String {
        self.file_type.clone()
    }

    pub fn bytes(&self) -> &[u8] {
        self.file_stream.as_slice()
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn size(&self) -> usize {
        self.file_size
    }



    pub fn write_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.file_stream)
    }
}
