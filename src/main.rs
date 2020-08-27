use std::env;
use std::fs;
use std::path::PathBuf;
use std::{io::{Read, copy}, process::Command};

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
    let bin_path = home_dir.join("bin");
    let deno_path = bin_path.join(file_name);
    println!("{:?}", deno_path);
    if deno_path.exists() {
        deno_path
    } else {
        download_deno(&bin_path, &version);
        deno_path
    }
}

fn download_deno(bin_path: &PathBuf, version: &String) {
    let target: String = "https://api.github.com/repos/denoland/deno/releases/tags/".to_owned() + version;
    let response = ureq::get(&target).call();
    println!("{:?}", &response);
    let mut reader = response.into_reader();
    
    let mut dest = {
        let fname = "deno_".to_string() + version;

        println!("file to download: '{}'", fname);
        let fname = bin_path.join(fname);
        println!("will be located under: '{:?}'", fname);
        fs::File::create(fname).expect("cant create")
    };

    copy(&mut reader, &mut dest).expect("cant copy");
}
