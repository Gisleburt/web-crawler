use structopt::StructOpt;
use web_crawler::crawl;

#[derive(Debug, StructOpt)]
struct Opts {
    /// The url to crawl
    url: String,
}

fn main() {
    let opts = Opts::from_args();
    dbg!(crawl(&opts.url));
}
