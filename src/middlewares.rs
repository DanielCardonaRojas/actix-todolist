use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use slog::info;
use std::pin::Pin;
use std::task::{Context, Poll};

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
pub struct Logging {
    logger: slog::Logger,
}

impl Logging {
    pub fn new(logger: slog::Logger) -> Logging {
        Logging { logger }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
    logger: slog::Logger,
}

impl<S, B> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start_time = chrono::Utc::now();
        let logger = self.logger.clone();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let req = res.request();
            let end_time = chrono::Utc::now();
            let duration = end_time - start_time;

            info!(
                logger,
                "{} {} {} response time: {}",
                req.method(),
                req.path(),
                res.status().as_u16(),
                duration.num_milliseconds(),
            );
            Ok(res)
        })
    }
}
