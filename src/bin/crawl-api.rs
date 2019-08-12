use actix_web::{middleware, web, App, Error as ActixError, HttpResponse, HttpServer};
use dotenv::dotenv;
use futures::Future;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::error::Error;
use std::str::Utf8Error;
use web_crawler::crawl;

struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Config {
        let _ = dotenv();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(80);

        let host = if std::env::var("PRODUCTION").is_ok() {
            "0.0.0.0".into()
        } else {
            "127.0.0.1".into()
        };

        Config { host, port }
    }
}

#[derive(Deserialize)]
struct CrawlInfo {
    domain: String,
}

impl CrawlInfo {
    fn get_domain(&self) -> Result<String, Utf8Error> {
        percent_decode_str(&self.domain)
            .decode_utf8()
            .map(|cow| cow.into_owned())
    }
}

fn get_crawl_data(
    info: web::Path<CrawlInfo>,
) -> impl Future<Item = HttpResponse, Error = ActixError> {
    web::block(move || {
        crawl(info.get_domain().map_err(|_e| "Could not parse domain")?)
            .map_err(|_e| "Could not parse domain")
    })
    .from_err()
    .map(|summary| HttpResponse::Ok().json(summary))
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_env();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .route("/{domain}", web::get().to_async(get_crawl_data))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()?;

    Err("Server closed unexpectedly".into())
}
