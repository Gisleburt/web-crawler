use actix_web::{middleware, web, App, Error as ActixError, HttpResponse, HttpServer};
use futures::Future;
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::str::Utf8Error;
use web_crawler::crawl;

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

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .route("/{domain}", web::get().to_async(get_crawl_data))
    })
    .bind(format!("127.0.0.1:8080"))
    .unwrap()
    .run()
    .unwrap();
}
