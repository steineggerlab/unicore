use crate::envs::variables::verbosity as system_verbosity;
pub fn print_message(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        print!("{}", msg);
    }
}
pub fn println_message(msg: &String, verbosity: u8) {
    if verbosity <= system_verbosity() {
        println!("{}", msg);
    }
}