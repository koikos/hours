use exitfailure::ExitFailure;
use log::LevelFilter;
use simple_error::SimpleError;
use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod cli;
mod time;
mod use_cases;

fn main() -> Result<(), ExitFailure> {
    // todo: add --verbose for printing debug information (ERROR level as default without --verbose)
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .init()
        .unwrap();
    let args = cli::Cli::from_args();

    //todo: how to put arguments parsing errors into exitcodes?
    match use_case_picker(&args.time) {
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

fn use_case_picker(input: &String) -> Result<String, SimpleError> {
    use regex::Regex;
    let re_hhhmmss = Regex::new(r"^\d*:\d*:?\d*$").unwrap();
    let re_hhhdddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

    return if re_hhhmmss.is_match(input) {
        use_cases::convert_time_to_decimal(input)
    } else if re_hhhdddd.is_match(input) {
        use_cases::convert_decimal_to_time(input)
    } else {
        Err(SimpleError::new("Couldn't match input to conversion."))
    };
}

//todo: add tests for exitcodes... external tests?
