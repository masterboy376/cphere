// src/middleware/rate_limiter.rs

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, Duration};

#[derive(Debug)]
pub enum RateLimiterError {
    TooManyRequests,
}

pub struct RequestContext {
    pub ip_address: String,
}

pub struct RateLimiter {
    pub requests: Mutex<HashMap<String, (SystemTime, u32)>>,
    pub max_requests: u32,
    pub window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        RateLimiter {
            requests: Mutex::new(HashMap::new()),
            max_requests,
            window,
        }
    }
}

#[async_trait]
pub trait Middleware {
    async fn handle(&self, ctx: &RequestContext) -> Result<(), RateLimiterError>;
}

#[async_trait]
impl Middleware for RateLimiter {
    async fn handle(&self, ctx: &RequestContext) -> Result<(), RateLimiterError> {
        let mut requests = self.requests.lock().unwrap();
        let now = SystemTime::now();
        let entry = requests.entry(ctx.ip_address.clone()).or_insert((now, 0));
        
        if now.duration_since(entry.0).unwrap_or(Duration::ZERO) > self.window {
            *entry = (now, 0);
        }
        
        entry.1 += 1;
        if entry.1 > self.max_requests {
            Err(RateLimiterError::TooManyRequests)
        } else {
            Ok(())
        }
    }
}
