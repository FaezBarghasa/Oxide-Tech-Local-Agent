use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::Error;
use oxide_security::SecurityManager;
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiKeyAuth {
    pub security: Arc<SecurityManager>,
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service,
            security: self.security.clone(),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: S,
    security: Arc<SecurityManager>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(str::to_string);

        let fut = self.service.call(req);
        let security = self.security.clone();

        Box::pin(async move {
            let token = token.ok_or_else(|| {
                actix_web::error::ErrorUnauthorized("Missing Bearer API token")
            })?;

            let hash = SecurityManager::hash_key(&token)
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid API key format"))?;

            // Check rate limit: 10,000 TPM standard capacity, consume 1
            if !security.check_rate_limit(&hash, 10_000, 1).await {
                return Err(actix_web::error::ErrorTooManyRequests("Rate limit exceeded for token"));
            }

            fut.await
        })
    }
}
