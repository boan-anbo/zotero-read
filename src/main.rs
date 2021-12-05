use std::fs;
use reqwest::{Error, Response};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use clap::{AppSettings, Parser};
use rust_embed::{RustEmbed};

#[derive(RustEmbed)]
#[folder = "scripts/"]
struct Asset;

#[derive(Parser)]
pub struct Args {
    #[clap(long, short)]
    pub view: bool,

    #[clap(long, short, default_value = "1")]
    pub range: usize,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let range_string = &args.range.to_string();
    let view_items : bool = args.view;

   // match &view_items { true => println!("true"), false => println!("false") }
    read_next_items(range_string.parse::<i32>().unwrap(), view_items).await;
}

async fn read_next_items(range: i32, view_items: bool) -> Result<Response, Error> {

    let range_string = range.to_string();

    let index_html = Asset::get("select_and_view_next_items.js").unwrap();

    let filedata = index_html.data.into_owned();


    let contents = std::str::from_utf8(&filedata).unwrap().to_string();

    println!("{}", contents);
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:23119/debug-bridge/execute?range={range_string}&view_items={view_items}&password=1234", range_string = range_string, view_items = view_items);
    println!("{}", &url);
    let res = client.post(url)
        .body(contents)
        .headers(construct_headers())
        .send().await?;
    println!("{}", res.status());
    Ok(res)

}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/javascript"));
    headers
}