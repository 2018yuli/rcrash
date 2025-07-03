mod opts;
mod opts_common;
mod opts_random;
mod processes;

pub use opts::{GenPasswordOpts, Opts, OutputFormat, SubCommand};
pub use opts_common::{OptsNew, SubCommandNew};
pub use opts_random::{OptsRandom, OutputFormatRandom, SubCommandRandom};
pub use processes::{process_csv, process_csv_common, process_generate_password};
