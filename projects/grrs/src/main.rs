use anyhow::{Context, Result};
use human_panic::setup_panic;
use log::{info, trace, warn};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{error::Error, thread, time::Duration};
use structopt::StructOpt;

#[macro_use]
extern crate log;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf, // PathBuf: 可跨平台使用的系统路径类型
}
fn main() -> Result<(), Box<dyn Error>> {
    // 处理ctrl+c
    // ctrlc::set_handler(move || println!("received Ctrl+C!")).expect("Error setting Ctrl-C handler");
    setup_panic!();
    panic!("hello world!");
    env_logger::init();
    let mut signals = Signals::new(&[SIGINT])?;
    info!("starting up");
    let args = Cli::from_args();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    warn!("warn");
    // read_to_string 将整个文件读取到内存中
    let content = read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line);
        }
    }
    pb.finish_with_message("done");
    trace!("1");

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    thread::sleep(Duration::from_secs(20));

    Ok(())
}
