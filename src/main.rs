use exitfailure::ExitFailure;
use simple_error::SimpleError;
use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod cli;
mod time;

fn main() -> Result<(), ExitFailure> {
    SimpleLogger::new();
    let args = cli::Cli::from_args();

    let result = picker(&args.time);

    //todo: how to put arguments parsing errors into exitcodes?
    match result {
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

fn picker(input: &String) -> Result<(String), SimpleError> {
    use regex::Regex;
    let re_hhhmmss = Regex::new(r"^\d*:\d*:?\d*$").unwrap();
    let re_hhhdddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

    return if re_hhhmmss.is_match(input) {
        let time = time::Time::from(&input)?;
        Ok(format!("{}", time.to_decimal()))
    } else if re_hhhdddd.is_match(input) {
        let time = time::Time::from(&input)?;
        Ok(time.to_string())
    } else {
        Err(SimpleError::new("Couldn't parse given time."))
    }
    //todo: add tests for this regex matcihng!
}

//todo: change mmm:ss to hhh:mm --> seconds are optional!