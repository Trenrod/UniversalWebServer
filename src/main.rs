use actix_web::web::Data;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

// Access paths
struct AppData {
    access_map: HashMap<String, String>,
}

#[get("/{filename:.*}")]
async fn index(app_data: Data<AppData>, req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load environment variables from .env file
    dotenv().expect(".env file not found");
    let mut access_map = HashMap::<String, String>::new();
    for (key, value) in env::vars() {
        if !key.starts_with("UWS") {
            continue;
        }

        if key.starts_with("UWS_ACCESS") {
            access_map.insert(key, value);
        }
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppData {
                access_map: access_map.clone(),
            }))
            .service(hello)
            .service(index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
