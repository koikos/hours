use structopt::StructOpt;
use exitfailure::ExitFailure;

/// Convert time from minutes and seconds to fraction of an hour, or vice versa.
/// Planned usage: hours $TIME
#[derive(Debug, StructOpt)]
struct Cli {
    /// Time in format hh:mm:ss, hh:mm, or hh,.....
    time: String,
}

fn main() -> Result<(), ExitFailure> {

    let args = Cli::from_args();
    println!("{:?}", &args.time);
    Ok(())
}
