use exitfailure::ExitFailure;
use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod cli;
mod time;

fn main() -> Result<(), ExitFailure> {
    SimpleLogger::new();
    let args = cli::Cli::from_args();
    let time = time::Time::from(&args.time);

    //todo: add presentation picker!

    //todo: how to put arguments parsing errors into exitcodes?
    match time {
        Ok(time) => {
            println!("{}", time);
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(exitcode::USAGE);
        }
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
