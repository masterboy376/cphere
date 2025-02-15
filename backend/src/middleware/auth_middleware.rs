use actix_service::{Service, Transform};
use actix_session::SessionExt;
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, Either, Ready};
use futures_util::future::LocalBoxFuture;

pub struct AuthMiddlewareFactory;

impl<S> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Either<
        Ready<Result<Self::Response, Self::Error>>,
        LocalBoxFuture<'static, Result<Self::Response, Self::Error>>,
    >;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();

        if let Ok(Some(_user_id)) = session.get::<String>("user_id") {
            // User is authenticated; proceed to the next service
            let fut = self.service.call(req);
            Either::Right(Box::pin(fut))
        } else {
            // User is not authenticated; return an Unauthorized response
            let (request, _payload) = req.into_parts();
            let response = HttpResponse::Unauthorized().finish();
            let service_response = ServiceResponse::new(request, response);
            Either::Left(ok(service_response))
        }
    }
}
