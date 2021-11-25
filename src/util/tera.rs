use tera::{Tera, Context};
use tera::Result;
use crate::util::path;


pub struct TeraEntity;

static mut TERA_ENTITY: Option<Tera> = None;

impl TeraEntity {
    pub fn init() {
        let p = path::this();
        let path = format!("{}/templates/**/*", p);
        let tera = match Tera::new(&path) {
            Ok(t) => t,
            Err(e) => panic!("{}", e)
        };
        unsafe { TERA_ENTITY = Some(tera) };
    }

    pub fn render(template_name: &str, context: &Context) -> Result<String> {
        let mut name = template_name.to_string();
        if !name.contains(".html") {
            name = format!("{}.html", &name)
        }
        unsafe {
            let s = match &TERA_ENTITY {
                None => String::new(),
                Some(v) => v.render(&name, &context)?,
            };
            Ok(s)
        }
    }

}