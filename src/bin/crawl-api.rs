use actix_web::{web, App, HttpResponse, HttpServer, middleware, Error as ActixError};
use web_crawler::crawl;
use serde::Deserialize;
use percent_encoding::percent_decode_str;
use std::str::Utf8Error;
use futures::Future;

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

fn get_crawl_data(info: web::Path<CrawlInfo>) -> impl Future<Item = HttpResponse, Error = ActixError> {
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
