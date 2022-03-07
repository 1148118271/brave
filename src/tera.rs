use tera::Tera;
use crate::path;

static mut TERA_ENTITY: Option<Tera> = None;

pub fn default() -> &'static Tera {
    unsafe {
        if TERA_ENTITY.is_none() {
            let p = path::default();
            let path = format!("{}/templates/**/*", p);
            let tera = match Tera::new(&path) {
                Ok(t) => t,
                Err(e) => panic!("{}", e)
            };
            TERA_ENTITY = Some(tera)
        }

        if let Some(v) = &TERA_ENTITY {
            return v;
        }

        panic!("加载页面模板异常！")

    }
}