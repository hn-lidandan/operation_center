use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimestampIdGenerator {
    last_timestamp: AtomicI64,
    counter: AtomicI64,
}

impl TimestampIdGenerator {
    pub fn new() -> Self {
        Self {
            last_timestamp: AtomicI64::new(0),
            counter: AtomicI64::new(0),
        }
    }

    pub fn next_id(&self) -> i64 {
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let last = self.last_timestamp.load(Ordering::Relaxed);
        if current_timestamp < last {
            panic!("Clock moved backwards! Refusing to generate ID.");
        }

        if current_timestamp == last {
            // 同一毫秒内递增计数器（限制在 0-999）
            let counter = self.counter.fetch_add(1, Ordering::SeqCst);
            last * 1000 + (counter % 1000)
        } else {
            // 新时间戳重置计数器
            self.counter.store(0, Ordering::SeqCst);
            self.last_timestamp
                .store(current_timestamp, Ordering::Relaxed);
            current_timestamp * 1000
        }
    }
}

// 全局ID生成器实例
static ID_GENERATOR: Lazy<TimestampIdGenerator> = Lazy::new(TimestampIdGenerator::new);

/// 生成下一个ID
pub fn next_id() -> i64 {
    ID_GENERATOR.next_id()
}
