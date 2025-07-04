pub mod cli;
mod processes;
pub mod utils;

pub use cli::{Base64SubCommand, Opts, SubCommand, TextSubCommand};

pub use processes::{
    process_csv, process_csv_common, process_decode, process_encode, process_generate_password,
};
