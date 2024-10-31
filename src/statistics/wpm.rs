use chrono::prelude::*;

pub fn calculate(word_count: usize, start_time: Option<DateTime<Local>>) -> f64 {
    if let Some(start) = start_time {
        let elapsed = Local::now() - start;
        let elapsed_seconds = elapsed.num_seconds() as f64;
        if elapsed_seconds > 0.0 {
            return (word_count as f64 / elapsed_seconds) * 60.0;
        }
    }
    0.0
}

