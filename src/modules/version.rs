use crate::envs::variables as var;
use crate::util::arg_parser::Args as Args;

pub fn run(_: &Args, _: var::BinaryPaths) {
    println!("{}", var::LOGO_ART);
    println!();
    println!("Unicore: Universal and efficient core gene phylogeny with Foldseek and ProstT5");
    println!("{} ver. {}", var::STABLE_FULL, var::VERSION);
    println!();
    println!("Developed by Dongwook Kim and Sukhwan Park");
    println!("Steinegger Lab, Seoul National University. 2024-");
    println!();
    println!("Contact  : endix1029@snu.ac.kr");
    println!("           pskvins@snu.ac.kr");
    println!("Corresp. : martin.steinegger@snu.ac.kr");
    println!();
}