// mod config {
//     pub use ::config::ConfigError;
//     use serde::Deserialize;
//     #[derive(Deserialize)]
//     pub struct Config {
//         pub server_addr: String,
//         pub pg: deadpool_postgres::Config,
//     }
//     impl Config {
//         pub fn from_env() -> Result<Self, ConfigError> {
//             let mut cfg = ::config::Config::new();
//             cfg.merge(::config::Environment::new())?;
//             cfg.try_into()
//         }
//     }
// }

// mod models {
//     use serde::{Deserialize, Serialize};
//     use tokio_pg_mapper_derive::PostgresMapper;

//     #[derive(Deserialize, PostgresMapper, Serialize)]
//     #[pg_mapper(table = "users")] // singular 'user' is a keyword..
//     pub struct User {
//         pub email: String,
//         pub first_name: String,
//         pub last_name: String,
//         pub username: String,
//     }

//     #[derive(Deserialize, PostgresMapper, Serialize)]
//     #[pg_mapper(table = "meta")] // singular 'user' is a keyword..
//     pub struct Meta {
//         pub start_time: Time,
//         pub network_name: String,
//     }
// }

// mod errors {
//     use actix_web::{HttpResponse, ResponseError};
//     use deadpool_postgres::PoolError;
//     use derive_more::{Display, From};
//     use tokio_pg_mapper::Error as PGMError;
//     use tokio_postgres::error::Error as PGError;

//     #[derive(Display, From, Debug)]
//     pub enum MyError {
//         NotFound,
//         PGError(PGError),
//         PGMError(PGMError),
//         PoolError(PoolError),
//     }
//     impl std::error::Error for MyError {}

//     impl ResponseError for MyError {
//         fn error_response(&self) -> HttpResponse {
//             match *self {
//                 MyError::NotFound => HttpResponse::NotFound().finish(),
//                 MyError::PoolError(ref err) => {
//                     HttpResponse::InternalServerError().body(err.to_string())
//                 }
//                 _ => HttpResponse::InternalServerError().finish(),
//             }
//         }
//     }
// }

// mod db {
//     use crate::{errors::MyError, models::User};
//     use deadpool_postgres::Client;
//     use tokio_pg_mapper::FromTokioPostgresRow;

//     pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
//         let _stmt = include_str!("../sql/add_user.sql");
//         let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
//         let stmt = client.prepare(&_stmt).await.unwrap();

//         client
//             .query(
//                 &stmt,
//                 &[
//                     &user_info.email,
//                     &user_info.first_name,
//                     &user_info.last_name,
//                     &user_info.username,
//                 ],
//             )
//             .await?
//             .iter()
//             .map(|row| User::from_row_ref(row).unwrap())
//             .collect::<Vec<User>>()
//             .pop()
//             .ok_or(MyError::NotFound) // more applicable for SELECTs
//     }

//     pub async fn get_meta(client: &Client, meta: Meta) -> Result<Meta, MyError> {
//         let _stmt = include_str!("../sql/test.sql");
//         let _stmt = _stmt.replace("$table_fields", &)
//     }
// }

// mod handlers {
//     use crate::{db, errors::MyError, models::User};
//     use actix_web::{web, Error, HttpResponse};
//     use deadpool_postgres::{Client, Pool};

//     pub async fn add_user(
//         user: web::Json<User>,
//         db_pool: web::Data<Pool>,
//     ) -> Result<HttpResponse, Error> {
//         let user_info: User = user.into_inner();

//         let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

//         let new_user = db::add_user(&client, user_info).await?;

//         Ok(HttpResponse::Ok().json(new_user))
//     }

//     pub async fn count_meta(
//         meta: web::Json<Meta>,
//         db_pool: web::Data<Pool>,   
//     ) -> Result<HttpResponse, Error> {
//         let meta: Meta = meta.into_inner();

//         let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

//         client.
//     }
// }

// use actix_web::{web, App, HttpServer};
// use dotenv::dotenv;
// use handlers::add_user;
// use tokio_postgres::NoTls;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();

//     let config = crate::config::Config::from_env().unwrap();
//     let pool = config.pg.create_pool(NoTls).unwrap();

//     let server = HttpServer::new(move || {
//         App::new()
//             .data(pool.clone())
//             .service(web::resource("/users").route(web::post().to(add_user)))
//     })
//     .bind(config.server_addr.clone())?
//     .run();
//     println!("Server running at http://{}/", config.server_addr);

//     server.await
// }


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
    .bind("127.0.0.1:3001")?
    .run()
    .await
}
