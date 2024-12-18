use std::process::Command as Cmd;
use std::env::current_dir;
use std::io::Write;
use std::fs::File;

fn error(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1)
}
fn run(cmd: &mut std::process::Command) {
    let cmdstr = format!("{:?}", cmd);
    if let Ok(mut child) = cmd.spawn() {
        let wait = child.wait();
        if let Ok(status) = wait {
            if let Some(code) = status.code() {
                match code {
                    0 => {}, x => error(&format!("Command exited with code {}\nCommand: {}", x, cmdstr)),
                }
            } else { error("Command was terminated by signal"); }
        } else { error("Could not get status"); }
    } else { error(&format!("Could not run command {}", cmdstr)); }
}

fn main() {
    // make bin directory
    run(Cmd::new("mkdir").arg("-p").arg("bin"));

    // write a script that runs target/{debug,release}/unicore
    let pwd = current_dir()
        .unwrap_or_else(|_| error("Could not get current directory"))
        .to_str()
        .unwrap_or_else(|| error("Could not convert path to string"))
        .to_string();
    let script_rls = format!("#!/bin/sh\n{pwd}/target/release/unicore $@\n");
    let script_dbg = format!("#!/bin/sh\n{pwd}/target/debug/unicore $@\n");

    let mut file = File::create("bin/unicore").unwrap();
    file.write_all(script_rls.as_bytes()).unwrap();
    let mut file = File::create("bin/unicore-debug").unwrap();
    file.write_all(script_dbg.as_bytes()).unwrap();

    // make the scripts executable
    run(Cmd::new("chmod").arg("+x").arg("bin/unicore"));
    run(Cmd::new("chmod").arg("+x").arg("bin/unicore-debug"));
}