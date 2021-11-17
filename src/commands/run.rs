use crate::handlers::{list_links, show_article, show_feed, show_sitemap, show_top_page};
use actix_files::Files;
use actix_web::web::get;
use actix_web::{App, HttpServer};

pub fn run() -> std::io::Result<actix_web::dev::Server> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", get().to(show_top_page))
            .route("/articles/{article_id}", get().to(show_article))
            .route("/feed.xml", get().to(show_feed))
            .route("/links", get().to(list_links))
            .route("/sitemap.txt", get().to(show_sitemap))
            .service(Files::new("/", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}
