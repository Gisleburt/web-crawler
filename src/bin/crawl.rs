use structopt::StructOpt;
use web_crawler::crawl;

#[derive(Debug, StructOpt)]
struct Opts {
    /// The url to crawl
    url: String,
}

fn main() {
    let opts = Opts::from_args();
    match crawl(&opts.url) {
        Ok(summary) => summary
            .into_iter()
            .for_each(|v| println!("{} - {}", v.url, v.count)),
        Err(e) => eprintln!("Something went wrong\n{:#?}", e),
    };
}
