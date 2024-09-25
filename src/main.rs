mod cmd;
mod utils;
mod process;
mod loger;
mod error;
mod report;

use clap::Parser;
use log::info;
use cmd::get_cmd;
use error::NemoError;
use process::statfq;
use report::summary;
use std::time::Instant;


fn main() -> Result<(), NemoError>{
    let cmd = cmd::Args::parse();
    let cmd_txt = get_cmd(cmd.clone());
    loger::logger(cmd.verbose, cmd.logfile, cmd.quiet)?;
    let start = Instant::now();

    let (content, length_hash, gc_hash )= statfq(cmd.input)?;
    summary(content, length_hash, gc_hash, &cmd.html, cmd_txt)?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
