use std::rc::Rc;

use actix_service::Transform;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error, Error, HttpMessage,
};
use futures::{
    future::{ready, LocalBoxFuture, Ready},
    FutureExt,
};

use crate::auth::{auth_with_file, auth_with_db};

pub type AuthenticationInfo = Rc<bool>;
pub struct AuthenticateMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticateMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointers so we can move them into the async block.
        let srv = self.service.clone();

        async move {
            // See if we can match it to a user.
            let auth = authenticate(&req).await;
            if auth {
                // If we found a user, add it to the request extensions
                // for later retrieval.
                req.extensions_mut()
                    .insert::<AuthenticationInfo>(Rc::new(auth));
            } else {
                return Err(error::ErrorUnauthorized("Authentication failed."));
            }

            let res = srv.call(req).await?;

            Ok(res)
        }
        .boxed_local()
    }
}

pub struct AuthenticateMiddlewareFactory {}

impl AuthenticateMiddlewareFactory {
    pub fn new() -> Self {
        AuthenticateMiddlewareFactory {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthenticateMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticateMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateMiddleware {
            service: Rc::new(service),
        }))
    }
}

async fn authenticate(req: &ServiceRequest) -> bool {
    let header_hash = req.headers().get("x-rustybot-hash");
    let header_salt = req.headers().get("x-rustybot-salt");
    let header_id = req.headers().get("x-rustybot-id");
    if header_hash.is_some() && header_salt.is_some() && header_id.is_some() {
        let hash = header_hash.unwrap().to_str().unwrap();
        let salt = header_salt.unwrap().to_str().unwrap();
        let id = header_id.unwrap().to_str().unwrap();
        // auth_with_file(id, hash, salt)
        auth_with_db(id, hash, salt).await
    } else {
        false
    }
}
