use anyhow::{anyhow, Result};
use clap::{AppSettings, Clap};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "liutsing <liutsingluo@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}
#[derive(Clap, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}
#[derive(Clap, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}
fn print_header(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),
        _ => println!("{}", body),
    }
}
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_header(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    println!("{:?}", &mime);
    print_body(mime, &body);
    Ok(())
}
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}
fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWER-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust httpie".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
