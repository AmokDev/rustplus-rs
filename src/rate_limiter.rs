use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RateLimiter {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
    }

    pub fn check_wait(&mut self, amount: f64) -> Duration {
        self.refill();
        if self.tokens >= amount {
            Duration::ZERO
        } else {
            let needed = amount - self.tokens;
            Duration::from_secs_f64(needed / self.refill_rate)
        }
    }

    pub fn consume(&mut self, amount: f64) {
        self.tokens -= amount;
    }
}
