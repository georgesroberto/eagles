use actix_files as fs; 
use actix_web::{App, HttpServer, web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub mod models; 
pub mod db;
pub mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = "db.sqlite";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Add the database pool to App
            .route("/departments", web::get().to(controllers::get_departments))
            .route("/doctors", web::get().to(controllers::get_doctors))
            .route("/patients", web::get().to(controllers::get_patients))
            .route("/consultations", web::get().to(controllers::get_consultations))
            .route("/chats", web::get().to(controllers::get_chats))
            
            .service(fs::Files::new("/assets", "./src/eagles_frontend/assets").show_files_listing()) // Serve assets
            .route("/", web::get().to(|| {
                HttpResponse::Ok().content_type("text/html").body(include_str!("../../eagles_frontend/index.html"))
            }))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}