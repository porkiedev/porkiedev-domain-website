//
// Contains code used by multiple binaries
//

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use tracing::info;

/// The request logging middleware
pub struct Logger;
impl <S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware { service }))
    }
}

/// The inner part of the request logging middleware
pub struct LoggerMiddleware<S> {
    service: S
}
impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        // Get the request headers
        let headers = req.headers();
        let mut log_string = String::new();

        // Try to resolve the client's IP address
        // NOTE: Checks for a cloudflare proxy header first (cf-connecting-ip),
        // and then checks for direct client address
        let client_ip_address = || {
            if let Some(client_ip) = headers.get("cf-connecting-ip") {
                return format!("{:?}", client_ip);
            }
            if let Some(client_ip) = req.peer_addr() {
                return format!("{}", client_ip);
            }
            String::from("unknown-ip")
        };

        // Format the string
        log_string.push_str(
            &format!("Incoming connection from '{}'\nRequest Headers:\n",
                     client_ip_address()
            )
        );

        // Push every request header to the string
        for header in headers {
            log_string.push_str(&format!("  {}: {:?}\n", header.0, header.1));
        }

        // Log the HTTP request
        info!("{log_string}");

        // Call the next service
        let fut = self.service.call(req);

        // Wait for the request to complete
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
