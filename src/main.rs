use actix_web::{get, App, HttpResponse, HttpServer, Responder,HttpRequest};
use std::collections::HashMap;
use std::env;
use std::io;

fn headers(req: HttpRequest) -> HashMap<String, String>{
    let map = &req.head().headers;
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for (key, value) in map {
        hashmap.insert(key.as_str().to_string(), value.to_str().unwrap().to_string());
    }
    hashmap
}
fn hostname() -> io::Result<()> {
    let name = hostname::get()?;
    println!("{}", name.to_string_lossy());
    Ok(())
}
#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    let hashmap = headers(req);
    hostname().unwrap();
    let serialized = serde_json::to_string(&hashmap).unwrap();
    HttpResponse::Ok().append_header(("content-type", "application/json")).body(serialized)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::NotFound()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let key: &str = "PING_LISTEN_PORT";
    let port:u16 = match env::var(key) {
    Ok(val) => val.parse().unwrap(),
    Err(_) => 8080,
    };

    HttpServer::new(|| {
        App::new()
            .service(ping)

    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}