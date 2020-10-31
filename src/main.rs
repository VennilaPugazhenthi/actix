use actix_files as fs;
use actix_web::{get, App, Error, HttpRequest, HttpServer};
use std::path::PathBuf;


#[get("/")]
async fn index(_req: HttpRequest) -> Result<fs::NamedFile, Error>{
    let path:PathBuf = "src/static/index.html".parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().route("/",web::get().to(index)))
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


