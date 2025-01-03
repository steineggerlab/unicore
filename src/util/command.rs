use crate::envs::error_handler as err;

pub fn run(cmd: &mut std::process::Command) {
    let cmdstr = format!("{:?}", cmd);
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