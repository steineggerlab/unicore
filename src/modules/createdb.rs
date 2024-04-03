use std::ops::Deref;
use crate::util::arg_parser::Commands::Createdb;
use crate::envs::error_handler as err;
use crate::util::run_external as run_ext;

pub fn run(args: &crate::util::arg_parser::Args, bin: &crate::envs::variables::BinaryPaths) -> Result<(), Box<dyn std::error::Error>> {
    // From args, retrieve the arguments for Createdb
    // If the arguments are not present, return an error
    match &args.command{
        Some(Createdb { input, output_db}) => {
            let input: String = input.clone().unwrap().display().to_string();
            let input_3di: String = input.clone() + ".3Di";
            let output_db: String = output_db.clone().unwrap().display().to_string();
            let output_db_3di: String = output_db.clone() + "_ss";
            // Run python script
            let mut cmd = std::process::Command::new("python3");
            let mut cmd = cmd.arg("/home/sukhwan/unicore/py/src/util/predict_3Di_encoderOnly.py")
                .arg("-i").arg(&input).arg("-o").arg(&input_3di).arg("--model").arg("/home/endix/git/ProstT5/model/")
                .arg("--half").arg("0");
            run_ext::run(&mut cmd);
            // Build foldseek db
            let foldseek_path = &bin.get("foldseek").unwrap().get_path();
            let mut cmd = std::process::Command::new(&foldseek_path);
            let mut cmd = cmd.arg("base:createdb").arg(&input).arg(&output_db)
                .arg("--shuffle").arg("0");
            run_ext::run(&mut cmd);
            let mut cmd = std::process::Command::new(&foldseek_path);
            let mut cmd = cmd.arg("base:createdb").arg(&input_3di).arg(&output_db_3di)
                .arg("--shuffle").arg("0");
            run_ext::run(&mut cmd);
        },
        _ => { err::error(err::ERR_GENERAL, Some("Weird arguments for Createdb".to_string())); },
    };
    Ok(())
}