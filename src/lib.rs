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

pub mod users {
    pub mod user;
}
pub mod dbconnection;

pub mod create_user {
    pub mod create_user;
}

pub mod create_table;

pub mod get_user {
   pub mod getuser;
    pub mod getallusers;
}