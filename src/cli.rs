use structopt::StructOpt;

/// Convert time from minutes and seconds to fraction of an hour, or vice versa.
/// Planned usage: hours $TIME
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Time in format hh:mm:ss, hh:mm, or hh,.....
    pub time: String,
}
