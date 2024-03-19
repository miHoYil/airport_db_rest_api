/*#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;*/


use dotenv::dotenv;

use std::env;

mod db;
mod error_handler;

// Классы таблиц из БД + методы работы с ними
mod schema;
mod airplanes;
mod airports;
mod clients;
mod airplane_seat_types;
mod plane_seat_connections;
mod flights_schedule;
mod tickets;

mod users;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use listenfd::ListenFd;

//use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    dotenv().ok();
	println!("DotEnv Read: Success");
    db::init();
	println!("Connection to DB: Success");	
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
		App::new()
		.wrap(
				Cors::permissive()
                // Cors::default()
					// .send_wildcard()
                   //// .allowed_origin("localhost:4004")
                    // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
					// .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    // .allowed_header(header::CONTENT_TYPE)
            )
		 .configure(airplanes::init_routes)
		 .configure(airports::init_routes)
		 .configure(clients::init_routes)
		 .configure(airplane_seat_types::init_routes)
		 .configure(plane_seat_connections::init_routes)
		 .configure(flights_schedule::init_routes)
		 .configure(tickets::init_routes)
		 .configure(users::init_routes)
	}
	);
	
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

	println!("Http server is starting...");

	server.run().await
}