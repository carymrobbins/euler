use std::time::SystemTime;

pub fn run<F : FnOnce() -> ()>(name: &str, f: F) {
    println!("{}", name);
    let start = SystemTime::now();
    f();
    let s = start.elapsed().map_or(
        "?".to_string(),
        |d| d.as_secs_f32().to_string()
    );
    println!("OK: Completed in {} seconds", s);
}
