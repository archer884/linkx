use core::fmt;
use std::{
    fmt::Display,
    io::{self, Read},
    process,
};

use clap::Parser;
use hashbrown::HashSet;
use scraper::{Html, Selector};

#[derive(Debug)]
enum Error<'a> {
    IO(io::Error),
    Scraper(scraper::error::SelectorErrorKind<'a>),
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(e) => e.fmt(f),
            Error::Scraper(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error<'_> {}

impl From<io::Error> for Error<'_> {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for Error<'a> {
    fn from(e: scraper::error::SelectorErrorKind<'a>) -> Self {
        Error::Scraper(e)
    }
}

/// A program for extracting links from HTML.
#[derive(Debug, Parser)]
struct Args {
    /// the base url used for relative links
    #[arg(short, long)]
    url: Option<String>,
    /// a style expression used to target specific links
    ///
    /// Defaults to just "a"
    #[arg(short, long)]
    style: Option<String>,
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Error> {
    let text = read_input()?;
    let document = Html::parse_fragment(&text);
    let selector = Selector::parse(args.style.as_deref().unwrap_or("a"))?;

    let mut duplicate_filter = HashSet::new();

    let links = document
        .select(&selector)
        .filter_map(|link| link.attr("href"))
        .filter(|&href| duplicate_filter.insert(href));

    if let Some(url) = args.url.as_ref() {
        display_with_url(url, links);
    } else {
        display(links);
    }

    Ok(())
}

fn display(links: impl IntoIterator<Item: Display>) {
    for link in links {
        println!("{link}");
    }
}

fn display_with_url(url: &str, links: impl IntoIterator<Item: Display>) {
    for link in links {
        println!("{url}{link}");
    }
}

fn read_input() -> io::Result<String> {
    let mut buf = Vec::new();
    io::stdin().lock().read_to_end(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into())
}
