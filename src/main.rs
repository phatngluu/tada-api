use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use postgres::{Client, NoTls};
use chrono::{NaiveDateTime};
use dotenv;

struct Meta {
    _id: i64,
    start_time: NaiveDateTime,
    network_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let conn_str = dotenv::var("DB_CONN").unwrap();

    let mut client = Client::connect(
        &conn_str,
        NoTls,
    ).unwrap();

    for row in client.query("SELECT * FROM public.meta", &[]).unwrap() {
        let meta = Meta {
            _id: row.get(0),
            start_time: row.get(1),
            network_name: row.get(2),
        };

        println!("{} | {} | {}", meta._id, meta.start_time, meta.network_name);
    }

    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(dotenv::var("LISTENING_ADDR").unwrap())?
    .run()
    .await
}
