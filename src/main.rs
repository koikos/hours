use exitfailure::ExitFailure;
use structopt::StructOpt;

mod cli;
mod time_converter;

fn main() -> Result<(), ExitFailure> {
    let args = cli::Cli::from_args();
    let result = time_converter::convert_time(&args.time);

    // todo: how to put arguments parsing into exitcodes?
    match result {
        Ok(result) => {
            println!("{}", result);
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::USAGE);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
