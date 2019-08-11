use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use web_crawler::crawl;
use serde::Deserialize;
use percent_encoding::percent_decode_str;
use std::str::Utf8Error;
use serde_json::json;

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


fn get_crawl_data(info: web::Path<CrawlInfo>) -> HttpResponse {
    if let Ok(domain) = info.get_domain() {
        crawl(&domain)
            .map(|summary| HttpResponse::Ok().json(summary))
            .unwrap_or_else(|_| HttpResponse::BadRequest().json(json!({
                "error": "Could not parse domain"
            })))
    } else {
        HttpResponse::BadRequest().json(json!({
            "error": "Could not parse domain"
        }))
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .route("/{domain}", web::get().to(get_crawl_data))

    })
        .bind(format!("127.0.0.1:8080"))
        .unwrap()
        .run()
        .unwrap();
}
