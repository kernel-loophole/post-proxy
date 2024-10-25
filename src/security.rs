use bcrypt::{hash, verify};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct Security {
    attempts: Arc<Mutex<HashMap<String, (u32, Instant)>>>,

    max_attempts: u32,
    lockout_duration: Duration,
}

impl Security {
    pub fn new(max_attempts: u32, lockout_duration_secs: u64) -> Self {
        Self {
            attempts: Arc::new(Mutex::new(HashMap::new())),
            max_attempts,
            lockout_duration: Duration::from_secs(lockout_duration_secs),
        }
    }
    pub fn verify_password(&self, password: &str, hashed: &str) -> bool {
        verify(password, hashed).unwrap_or(false)
    }
    pub fn check_ip_allowed(&self, ip: &str, allowed_ip: &str) -> bool {
        ip == allowed_ip
    }
    pub fn is_rate_limited(&self, username: &str) -> bool {
        let mut attempts = self.attempts.lock().unwrap();

        if let Some(&(count, timestamp)) = attempts.get(username) {
            if count >= self.max_attempts {
                if timestamp.elapsed() < self.lockout_duration {
                    return true; // User is still locked out
                } else {
                    attempts.remove(username); // Reset after lockout duration
                }
            }
        }

        false // User is not rate limited
    }

    // Record a failed login attempt
    pub fn record_attempt(&self, username: &str) {
        let mut attempts = self.attempts.lock().unwrap();

        let entry = attempts.entry(username.to_string()).or_insert((0, Instant::now()));
        entry.0 += 1; // Increment attempt count
    }
}
