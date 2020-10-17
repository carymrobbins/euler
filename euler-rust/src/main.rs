extern crate lazy_static;

use std::collections::BTreeMap;
use std::env;
use std::process;
use std::string::String;
use std::vec::Vec;

mod euler001;
mod euler002;
mod euler005;
mod euler006;
mod euler007;
mod primes;
mod run;

fn mk_euler_tasks() -> BTreeMap<u16, fn()> {
    let mut res: BTreeMap<u16, fn()> = BTreeMap::new();
    res.insert(001, euler001::main);
    res.insert(002, euler002::main);
    res.insert(005, euler005::main);
    res.insert(006, euler006::main);
    res.insert(007, euler007::main);
    res
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // Skip the program name.
    for arg in args.iter() {
        if arg == "--help" {
            println!("{}", USAGE);
            process::exit(0);
        }
    }
    if args.is_empty() {
        match env::var_os("EULER") {
            None => abort(format!("Missing arguments\n{}", USAGE)),
            Some(os_str) => match os_str.to_str() {
                None => abort(format!("Failed to decode EULER env var")),
                Some(var) => args = var.split_whitespace().map(|x| x.to_string()).collect(),
            },
        }
    }

    let euler_tasks = mk_euler_tasks();
    let get_task = |k: &String| match k.parse::<u16>() {
        Err(_) => abort(format!("Number not accepted: {}", k)),
        Ok(n) => match euler_tasks.get(&n) {
            None => abort(format!("Euler not found: {:03}", n)),
            Some(f) => *f,
        },
    };

    let tasks: Vec<fn()> = if args == ["all"] {
        euler_tasks.values().map(|f| *f).collect()
    } else {
        args.iter().map(get_task).collect()
    };

    tasks.iter().for_each(|f| f());
    println!("Complete!");
}

static USAGE: &str = "Usage: euler-rust <000> [<000> ...]";

fn abort<T>(msg: String) -> T {
    eprintln!("{}", msg);
    process::exit(1)
}
