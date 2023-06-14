use std::{io::{BufReader, BufRead}, fs::File};


fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    match args.nth(1) {
        None => on_error("no file provided", anyhow::Error::msg("no file provided")),
        Some(filename) => {
            let mut file = open(&filename)?;
            let mut text = String::new();
            let _ = file.read_to_string(&mut text)?;
            let tree = parse_wiki_text::Configuration::default().parse(&text);
            let mut links = vec![];
            get_wiki_links::get_links(&mut links, &tree.nodes);
            println!("{links:?}");
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

