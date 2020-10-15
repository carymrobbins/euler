use std::time::SystemTime;

pub fn run<F: FnOnce() -> ()>(name: &str, f: F) {
    println!("Running Euler {} ...", name);
    let start = SystemTime::now();
    f();
    let s = start
        .elapsed()
        .map_or("?".to_string(), |d| format!("{:.6}", d.as_secs_f32()));
    println!("  ✔ OK: Completed in {} seconds", s);
    println!("----------------------------------------");
}
