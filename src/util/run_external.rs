use crate::envs::error_handler as err;

pub fn run(cmd: &mut std::process::Command) {
    if let Ok(mut child) = cmd.spawn() {
        let wait = child.wait();
        if let Ok(status) = wait {
            if let Some(code) = status.code() {
                match code {
                    0 => {},
                    x => err::error(x, Some("Command failed".to_string())),
                }
            } else {
                err::error(err::ERR_GENERAL, Some("Command was terminated by signal".to_string()));
            }
        } else {
            err::error(err::ERR_GENERAL, Some("Could not get status".to_string()));
        }
    } else {
        err::error(err::ERR_GENERAL, Some("Could not run command".to_string()));
    }
}