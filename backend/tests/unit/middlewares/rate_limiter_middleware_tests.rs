// tests/unit/rate_limiter_test.rs

use cphere_backend::middleware::rate_limiter::{RateLimiter, Middleware as RateLimiterMiddleware, RateLimiterError, RequestContext};
use std::time::Duration;

#[actix_web::test]
async fn test_rate_limiter_allows_requests() {
    let limiter = RateLimiter::new(5, Duration::from_secs(60));
    let ctx = RequestContext { ip_address: "127.0.0.1".to_string() };

    for i in 0..5 {
        let res = limiter.handle(&ctx).await;
        assert!(res.is_ok(), "Request {} should pass", i+1);
    }
}

#[actix_web::test]
async fn test_rate_limiter_exceeds_requests() {
    let limiter = RateLimiter::new(3, Duration::from_secs(60));
    let ctx = RequestContext { ip_address: "127.0.0.1".to_string() };

    for _ in 0..3 {
        let res = limiter.handle(&ctx).await;
        assert!(res.is_ok());
    }
    let res = limiter.handle(&ctx).await;
    assert!(matches!(res, Err(RateLimiterError::TooManyRequests)));
}
