use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.get(1) {
        Some(command) => match command.as_str() {
            "help" => help(),
            "--help" => help(),
            "-h" => help(),
            "--version" => version(),
            _ => deno_run(&args.split_at(1).1.to_vec()),
        },
        _ => deno_run(&Vec::new()),
    }
}

fn help() {
    deno_run(&vec!["help".to_owned()])
}

fn version() {
    let devo_version = env!("CARGO_PKG_VERSION");
    println!("devo {}", devo_version);
    deno_run(&vec!["--version".to_owned()]);
}

fn deno_run(args: &Vec<String>) {
    Command::new("deno")
        .args(args)
        .spawn()
        .expect("deno failed to start");
}
