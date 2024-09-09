use crate::envs::variables::verbosity as system_verbosity;
pub fn print(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        println!("{}", msg);
    }
}