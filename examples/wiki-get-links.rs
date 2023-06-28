use std::{io::{BufReader, BufRead}, fs::File};
use futures_time::prelude::*;
use parking_lot::RwLock;
use std::sync::Arc;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    match args.nth(1) {
        None => on_error("no file provided", anyhow::Error::msg("no file provided")),
        Some(filename) => {
            let mut file = open(&filename)?;
            let text = Arc::pin(RwLock::new(String::new()));
            let _ = file.read_to_string(&mut text.write())?;
            let tree: Arc<RwLock<Option<async_parse_wiki_text::Output<'_>>>> = Default::default();
            let t = tree.clone();
            let te = text.clone();
            match async move {
                    *t.write() = Some(async_parse_wiki_text::Configuration::default().parse(&te.read()).await);
                }
                .timeout(futures_time::time::Duration::from_secs(3))
                .await  
            {
                Ok(result) => {
                    let mut links = vec![];
                    wiki_get_links::get_links(&mut links, &tree.read().unwrap().nodes);
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

