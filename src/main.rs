use std::{collections::VecDeque, fs::read_to_string, thread, time::Duration};

enum TestEnum {
    Integer(u64),
    MyString(String)
}
fn get_measurement() -> (u64, u64) {
    let file: String = read_to_string("/proc/stat").unwrap();
    let cpu_line = file
        .lines()
        .filter(|l| l.starts_with("cpu "))
        .next()
        .unwrap();
    let fields: Vec<u64> = cpu_line
        .split_whitespace()
        .skip(1)
        .take(4)
        .map(|f| f.parse().unwrap())
        .collect();
    let busy_tics: u64 = fields.iter().take(3).sum();
    let idle_tics: u64 = fields.iter().skip(3).sum();
    (busy_tics, idle_tics)
}

fn main() {
    let ref_points = 10;
    let sample_interval = Duration::from_millis(100);

    let mut cache: VecDeque<(u64, u64)> = (0..ref_points)
        .into_iter()
        .map(|_| {
            thread::sleep(sample_interval);
            get_measurement()
        })
        .collect();
    loop {
        let sample1 = cache.pop_front().unwrap();
        let sample2 = cache.iter().next_back().unwrap();

        let total_busy = sample2.0 - sample1.0;
        let total_idle = sample2.1 - sample1.1;
        std::fs::write(
            "/tmp/cpu_usage",
            format!(
                "{:.2}%",
                total_busy as f64 / (total_idle + total_busy) as f64 * 100.
            ),
        ).unwrap();
        thread::sleep(sample_interval);
        cache.push_back(get_measurement());
    }
}
