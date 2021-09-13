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
    } else if let Some(_) = matches.subcommand_matches("release") {
        release();
    } else if let Some(_) = matches.subcommand_matches("push") {
        push();
    }
}
fn run(command_vec: &Vec<&str>, dir: &str) {
    let mut command = Command::new(command_vec[0]);
    for arg_i in 1..command_vec.len() {
        command.arg(command_vec[arg_i]);
    }
    command.current_dir(dir);
    println!("$ {}", command_vec.join(" "));
    if let Ok(mut child) = command.spawn() {
        if child
            .wait()
            .expect("command failed")
            .code()
            .unwrap()
            == 0
        {
            println!("  [ok]");
        } else {
            println!("  [error]");
        }
    } else {
        println!("command failed");
    }
}
fn push() {
    let commands: Vec<Vec<&str>> = vec![
        vec!["git", "add", "."],
        vec!["git", "commit", "-m", "m push"],
        vec!["git", "push"]
    ];
    println!("plop");
    for command in commands.iter() {
        run(command, "/Users/loicbourgois/github.com/mirovia/mirovia/")
    }
    // if let Ok(mut child) = Command::new("npm")
    //     .arg("install")
    //     .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/")
    //     .spawn()
    // {
    //     if child
    //         .wait()
    //         .expect("npm wasn't running")
    //         .code()
    //         .unwrap()
    //         == 0
    //     {
    //         println!("[ok]    npm install");
    //     } else {
    //         println!("[error] npm install");
    //     }
    // } else {
    //     println!("npm didn't start");
    // }
    // return false;
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
            "/Users/loicbourgois/github.com/mirovia/mirovia/front/src",
            RecursiveMode::Recursive,
        )
        .unwrap();
    watcher_front
        .watch(
            "/Users/loicbourgois/github.com/mirovia/mirovia/front/Cargo.toml",
            // docker run --rm --name mirovia -v /Users/loicbourgois/github.com/mirovia/mirovia/docs:/usr/share/nginx/html:ro -p 8080:80 -v /Users/loicbourgois/github.com/mirovia/mirovia/nginx.conf:/etc/nginx/nginx.conf:ro nginx
            RecursiveMode::Recursive,
        )
        .unwrap();
    watch_callback();
    loop {
        match rx.recv() {
            Ok(_event) => {
                watch_callback()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
fn watch_callback() {
    if test_front() {
        npm_install();
        build_front_wasm();
        release();
        println!("Done");
    }
}
fn npm_install() -> bool {
    if let Ok(mut child) = Command::new("npm")
        .arg("install")
        .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/front/")
        .spawn()
    {
        if child
            .wait()
            .expect("npm wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    npm install");
            return delete_release();
        } else {
            println!("[error] npm install");
        }
    } else {
        println!("npm didn't start");
    }
    return false;
}
fn release() -> bool {
    if let Ok(mut child) = Command::new("webpack")
        .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/front/")
        .spawn()
    {
        if child
            .wait()
            .expect("webpack wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    webpack");
            return delete_release();
        } else {
            println!("[error] webpack");
        }
    } else {
        println!("webpack build didn't start");
    }
    return false;
}
fn delete_release() -> bool {
    if let Ok(mut child) = Command::new("rm")
        .arg("-r")
        .arg("-f")
        .arg("/Users/loicbourgois/github.com/mirovia/mirovia/docs")
        .spawn()
    {
        if child
            .wait()
            .expect("rm wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    rm");
            return copy_release();
        } else {
            println!("[error] rm");
        }
    } else {
        println!("rm didn't start");
    }
    return false;
}
fn copy_release() -> bool {
    if let Ok(mut child) = Command::new("cp")
        .arg("-R")
        .arg("/Users/loicbourgois/github.com/mirovia/mirovia/front/dist")
        .arg("/Users/loicbourgois/github.com/mirovia/mirovia/docs")
        .spawn()
    {
        if child
            .wait()
            .expect("cp wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    cp");
            return copy_404();
        } else {
            println!("[error] cp");
        }
    } else {
        println!("cp didn't start");
    }
    return false;
}
fn copy_404() -> bool {
    if let Ok(mut child) = Command::new("cp")
        .arg("/Users/loicbourgois/github.com/mirovia/mirovia/docs/index.html")
        .arg("/Users/loicbourgois/github.com/mirovia/mirovia/docs/404.html")
        .spawn()
    {
        if child
            .wait()
            .expect("cp wasn't running")
            .code()
            .unwrap()
            == 0
        {
            println!("[ok]    cp");
            return true;
        } else {
            println!("[error] cp");
        }
    } else {
        println!("cp didn't start");
    }
    return false;
}

fn test_front() -> bool {
    if let Ok(mut child) = Command::new("cargo")
        .arg("test")
        .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/front/")
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
        .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/front/")
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
        .current_dir("/Users/loicbourgois/github.com/mirovia/mirovia/front/")
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
                "/Users/loicbourgois/github.com/mirovia/mirovia/{}",
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
