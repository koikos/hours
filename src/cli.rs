use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Convert time from minutes and seconds to fraction of an hour, or vice versa.
pub struct Cli {
    /// time in format hhh:mm:ss or hhh.dddd
    /// Examples:
    ///     hours 1:30    -> 1.500
    ///     hours 1:30:18 -> 1.505
    ///     hours 1.055   -> 1:03:18
    #[structopt(verbatim_doc_comment)]
    pub time: String,
}
