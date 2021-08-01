use clap::load_yaml;
use clap::App;
use std::process::Command;
extern crate notify;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(ref matches) = matches.subcommand_matches("watch") {
        watch(&WatchArgs {
            verbose: matches.is_present("verbose"),
        })
    } else if let Some(_) = matches.subcommand_matches("format") {
        format_code()
    } else if let Some(_) = matches.subcommand_matches("front") {
        run_front()
    }
}
struct WatchArgs {
    verbose: bool,
}
fn watch(x: &WatchArgs) {
    if x.verbose {
        println!("verbose on")
    }
    let (tx, rx) = channel();
    let mut watcher_front = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher_front
        .watch(
            "/Users/loicbourgois/github.com/gouttelettes/gouttelettes/front/src",
            RecursiveMode::Recursive,
        )
        .unwrap();
    watcher_front
        .watch(
            "/Users/loicbourgois/github.com/gouttelettes/gouttelettes/front/Cargo.toml",
            RecursiveMode::Recursive,
        )
        .unwrap();
    loop {
        match rx.recv() {
            Ok(_event) => {
                if test_front() {
                    build_front_wasm();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
fn test_front() -> bool {
    if let Ok(mut child) = Command::new("cargo")
        .arg("test")
        .current_dir("/Users/loicbourgois/github.com/gouttelettes/gouttelettes/front/")
        .spawn()
    {
        if child
            .wait()
            .expect("cargo test wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    cargo test");
            return true;
        } else {
            println!("[error] cargo test");
        }
    } else {
        println!("wasm-pack build didn't start");
    }
    return false;
}
fn run_front() {
    if let Ok(mut child) = Command::new("npm")
        .arg("start")
        .current_dir("/Users/loicbourgois/github.com/gouttelettes/gouttelettes/front/")
        .spawn()
    {
        if child
            .wait()
            .expect("npm start wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    npm start");
        } else {
            println!("[error] npm start");
        }
    } else {
        println!("wasm-pack build didn't start");
    }
}
fn build_front_wasm() {
    if let Ok(mut child) = Command::new("wasm-pack")
        .arg("build")
        .current_dir("/Users/loicbourgois/github.com/gouttelettes/gouttelettes/front/")
        .spawn()
    {
        if child
            .wait()
            .expect("wasm-pack build wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    wasm-pack build");
        } else {
            println!("[error] wasm-pack build");
        }
    } else {
        println!("wasm-pack build didn't start");
    }
}
fn format_code() {
    for name in ["front", "mir"] {
        let command_name_1 = format!("cargo fmt {}", name);
        if let Ok(mut child) = Command::new("cargo")
            .arg("fmt")
            .current_dir(format!(
                "/Users/loicbourgois/github.com/gouttelettes/gouttelettes/{}",
                name
            ))
            .spawn()
        {
            child
                .wait()
                .expect(format!("{} wasn't running", command_name_1).as_str());
            println!("[ok] {}", command_name_1);
        } else {
            println!("{} didn't start", command_name_1);
        }
    }
}
