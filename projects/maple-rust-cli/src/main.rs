use clap::Parser;
use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};
use std::fs::{self};
use std::io::{self, stdout, Write};
use std::path::Path;
use std::result::Result as IOResult;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    path: String,
}

/**
 * 1、读取目的文件夹内的文件
 * 2、cli展示1x图的文件名
 * 3、选择文件，修改文件名，同时更新2x、3x图的文件名
 * 4、配合日志/进度条输出
 */
fn main() -> Result<()> {
    let args = Args::parse();
    let entries = fs::read_dir(Path::new(&args.path))
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<IOResult<Vec<_>, io::Error>>()
        .unwrap();
    let files: Vec<&str> = entries
        .iter()
        .map(|f| f.to_str().unwrap())
        .filter(|f| !f.contains('@') && f.ends_with(".png"))
        .collect();

    println!("files: {:?}", files);
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styles text here."),
        ResetColor
    )?;

    Ok(())
}
