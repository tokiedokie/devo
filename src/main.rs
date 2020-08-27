use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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
    let deno_path = get_deno_path();
    Command::new(deno_path)
        .args(args)
        .spawn()
        .expect("deno failed to start");
}

fn get_deno_path() -> PathBuf {
    let version = fs::read_to_string("./.devo").unwrap_or_default();
    let file_name: String = "deno".to_string() + &version;
    let home_dir = match env::var("DENO_INSTALL") {
        Ok(path) => PathBuf::from(&path),
        Err(_) => dirs::home_dir().unwrap().join(".deno"),
    };
    let deno_dir_bin = home_dir.join("bin").join(file_name);
    println!("{:?}", deno_dir_bin);
    deno_dir_bin
}
