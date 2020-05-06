macro_rules! env_or {
    ($name:literal, $default:literal) => {
        std::env::var($name).unwrap_or($default.to_owned())
    }
}

use actix_cors::Cors;
use actix_web::{guard::Host, web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv()
        .expect("Unable to load environment variables.");

    let db_url = std::env::var("DATABASE_URL")
        .expect("No database url specified");
    let db_cm = ConnectionManager::<MysqlConnection>::new(db_url);
    let db_pool = r2d2::Pool::builder()
        .build(db_cm)
        .expect("Failed to create DB connection pool");

    HttpServer::new(move || App::new()
        .data(db_pool.clone())
        .service(
            web::scope("/")
                .wrap(
                    Cors::new()
                        .supports_credentials()
                        .finish()
                )
                .guard(Host(env_or!("API_HOSTNAME", "api.localhost")))
                .configure(showman_api::setup)
        )
    )
        .bind("0.0.0.0:80")?
        .run()
        .await
}
