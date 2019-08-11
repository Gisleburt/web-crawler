use web_crawler::crawl;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// The url to crawl
    url: String,
}

fn main() {
    let opts = Opts::from_args();
    crawl(&opts.url);
}
