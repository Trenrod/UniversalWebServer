use actix_web::web::Data;
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use actix_files::NamedFile;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// Access paths
struct AppData {
    access_map: HashMap<String, String>,
}

#[get("/{filename:.*}")]
async fn index(app_data: Data<AppData>, req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    // Redirect to root path
    let absolute_path = Path::new("/srv/public").join(path.clone());
    println!("Access path:{:?} absolute_path:{:?}", path, absolute_path);
    let file = NamedFile::open(absolute_path)?;
    Ok(file)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Universal WebServer started");
    // load optionally environment variables from .env file
    let _ = dotenv().inspect_err(|_| println!(".env not loaded"));
    let mut access_map = HashMap::<String, String>::new();
    for (key, value) in env::vars() {
        if !key.starts_with("UWS") {
            continue;
        }

        if key.starts_with("UWS_ACCESS") {
            access_map.insert(key, value);
        }
    }

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/certificates/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/etc/certificates/cert.pem")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Opener-Policy", "same-origin"))
                    .add(("Cross-Origin-Embedder-Policy", "require-corp")),
            )
            .app_data(Data::new(AppData {
                access_map: access_map.clone(),
            }))
            .service(hello)
            .service(index)
    })
    .bind_openssl("0.0.0.0:8000", builder)?
    .run()
    .await
}
