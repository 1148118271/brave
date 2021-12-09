use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use futures::Future;
use futures::future::{ok, Ready};

use crate::util::session;


const PATHS: [&str; 8] = ["/", "/list", "/admin", "/admin/login", "/static/*", "/files/*", "/blog/*", "/favicon.ico"];

pub struct Auth;

impl<S, B> Transform<S> for Auth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service))
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();
        log::info!("[{} -> {}]", req.method().as_str(), req.path());
        Box::pin(async move {
            let x = req.path();
            let validation = path_validation(x);
            if validation {
                return Ok(svc.call(req).await?)
            }

            let token = req.cookie("token");
            let token = match token {
                None => return Err(redirect()),
                Some(v) => v.value().to_owned(),
            };

            let option = session::get(&token);
            match option {
                None => return Err(redirect()),
                _ => {}
            }

            Ok(svc.call(req).await?)
        })

    }
}

/// 重定向到登录页面
fn redirect() -> Error {
    let mut response = HttpResponse::new(StatusCode::FOUND);
    let header: &mut HeaderMap = response.headers_mut();
    let k = HeaderName::from_bytes(b"location").unwrap();
    let v = HeaderValue::from_bytes(b"/admin").unwrap();
    header.append(k, v);
    Error::from(response)
}


fn path_validation(path: &str) -> bool {
    if PATHS.contains(&"*")
        || PATHS.contains(&"/*")
        || PATHS.contains(&path)
    {
        return true
    }

    for x in PATHS {
        if x.contains(&"*") {
            let p = x.replace("/*", "");
            if path.contains(&p) {
                return true
            }
        }
    }

    false
}