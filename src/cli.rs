use clap::ArgGroup;
use clap::Parser;

/// Repeatedly output a line with all specified STRING(s), or 'y'.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("thread_mode")
        .args(["threads", "cpu_threads"])
        .multiple(false)
))]
pub struct Args {
    /// The string(s) to output repeatedly (default: "y")
    pub strings: Vec<String>,

    /// Ring buffer capacity
    #[arg(long = "ring-capacity", default_value_t = 8)]
    pub ring_capacity: u32,

    /// Enable SQPOLL mode with an optionally specified idle time
    #[arg(long = "sqpoll", num_args(0..=1), default_missing_value = "1000")]
    pub sqpoll: Option<u32>,

    /// Number of threads to use
    #[arg(long = "threads", default_value_t = 1)]
    pub threads: usize,

    /// Use number of CPU cores as thread count
    #[arg(long = "cpu-threads")]
    pub cpu_threads: bool,
}
