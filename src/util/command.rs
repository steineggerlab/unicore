use crate::envs::error_handler as err;
use crate::util::message as msg;

pub fn run(cmd: &mut std::process::Command) {
    let cmdstr = format!("{:?}", cmd).replace("\"", "");
    msg::println_message(&format!("Running command: {}", cmdstr), 4);
    if let Ok(mut child) = cmd.spawn() {
        let wait = child.wait();
        if let Ok(status) = wait {
            if let Some(code) = status.code() {
                match code {
                    0 => {},
                    x => err::error(err::ERR_GENERAL, Some(format!("Command exited with code {}\nCommand: {}", x, cmdstr))),
                }
            } else {
                err::error(err::ERR_GENERAL, Some("Command was terminated by signal".to_string()));
            }
        } else {
            err::error(err::ERR_GENERAL, Some("Could not get status".to_string()));
        }
    } else {
        err::error(err::ERR_GENERAL, Some(format!("Could not run command {}", cmdstr)));
    }
}

pub fn run_code(cmd: &mut std::process::Command) -> i32 {
    let cmd = cmd.stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    let cmdstr = format!("{:?}", cmd).replace("\"", "");
    msg::println_message(&format!("Running command: {}", cmdstr), 4);
    if let Ok(mut child) = cmd.spawn() {
        let wait = child.wait();
        if let Ok(status) = wait {
            if let Some(code) = status.code() {
                code
            } else {
                1
            }
        } else {
            1
        }
    } else {
        1
    }
}

pub fn _run_at(cmd: &mut std::process::Command, path: &std::path::Path) {
    let cmdstr = format!("{:?}", cmd);
    if let Ok(mut child) = cmd.current_dir(path).spawn() {
        let wait = child.wait();
        if let Ok(status) = wait {
            if let Some(code) = status.code() {
                match code {
                    0 => {},
                    x => err::error(err::ERR_GENERAL, Some(format!("Command exited with code {}\nCommand: {}", x, cmdstr))),
                }
            } else {
                err::error(err::ERR_GENERAL, Some("Command was terminated by signal".to_string()));
            }
        } else {
            err::error(err::ERR_GENERAL, Some("Could not get status".to_string()));
        }
    } else {
        err::error(err::ERR_GENERAL, Some(format!("Could not run command {}", cmdstr)));
    }
}