use std::{io::{BufReader, BufRead}, fs::File};
use futures_time::prelude::*;
use parking_lot::RwLock;
use std::sync::Arc;

use async_parse_wiki_text::WikiText;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init();

    let mut args = std::env::args();
    match args.nth(1) {
        None => on_error("no file provided", anyhow::Error::msg("no file provided")),
        Some(filename) => {
            let mut file = open(&filename)?;
            let mut text = String::new();
            let _ = file.read_to_string(&mut text)?;
            let tree: Arc<RwLock<Option<async_parse_wiki_text::Output>>> = Default::default();
            let text = WikiText::new(text);
            let te = text.clone();
            let tr = tree.clone();
            match async move {
                    *tree.write() = Some(async_parse_wiki_text::Configuration::default().parse(te).await);
                }
                .timeout(futures_time::time::Duration::from_secs(11))
                .await  
            {
                Ok(_) => {
                    let mut links = vec![];
                    if let Some(ref output) = *tr.read() {
                        wiki_get_links::get_links(&mut links, &output.nodes);
                    }
                    println!("{links:?}");
                }
                _ => eprintln!("aborted..."),
            }
        }
    }
    Ok(())
}
fn on_error(msg: &str, err: anyhow::Error) {
    eprintln!("{msg} {:?}", err);
    std::process::exit(1);
}

fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

