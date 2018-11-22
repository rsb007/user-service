extern crate actix;
extern crate actix_web;
extern crate bytes;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate env_logger;
extern crate futures;
extern crate json;
extern crate listenfd;
extern crate maplit;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use actix_web::{
    App, http, HttpRequest,
    middleware, Responder, server,
};

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::frame::IntoBytes;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::rows::Row;
use listenfd::ListenFd;
use std::str;
extern crate userservice;



fn main() {
    {
        ::std::env::set_var("RUST_Log", "actix_web=info");
        env_logger::init();
        let con=userservice::dbconnection::connection();
        /* userservice::create_table::create_table(&con);
         println!("table created");*/
        let mut listenfd = ListenFd::from_env();
        let mut server = server::new(|| {
            App::new()
                //enable logger
                .middleware(middleware::Logger::default())
                .resource("/insert", |r| r.method(http::Method::POST).f(userservice::create_user::create_user::insert_manual))
                .resource("/getuser", |r| r.method(http::Method::GET).f(userservice::get_user::getuser::get_manual))
                .resource("/getalluser", |r| r.method(http::Method::GET).f(userservice::get_user::getallusers::get_manual))
        });

        server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            server.listen(l)
        } else {
            server.bind("127.0.0.1:8000").unwrap()
        };

        server.run();
    }
}
