use exitfailure::ExitFailure;
use structopt::StructOpt;

use crate::time_converter::convert_time;

mod time_converter;
mod cli;

fn main() -> Result<(), ExitFailure> {
    let args = cli::Cli::from_args();
    println!("{}", convert_time(&args.time));
    Ok(())
}
