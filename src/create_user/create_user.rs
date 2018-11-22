use dbconnection;
use cdrs::query::QueryExecutor;
use users::user::user;

use actix_web::{
    AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};

use futures::{Future, Stream};
use json::JsonValue;
use serde_json;
use std;
use json;

fn insert(row: user)
{
    let session = dbconnection::connection();

    let insert_struct_cql = "INSERT INTO auction.users \
                           (id, name) VALUES (?, ?)";
    session
        .query_with_values(insert_struct_cql, query_values!(row.id, row.name))
        .expect("insert error ");
}

pub fn insert_manual(
    req: &HttpRequest
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.payload()
        .concat2()
        .from_err()
        .and_then(|body| {
            // body is loaded, now we can deserialize json-rust
            let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
            /*let injson: JsonValue = match result {
                Ok(v) => v,
                Err(e) => object! {"err" => e.to_string() } ,
            };*/
            let injson: JsonValue = result.unwrap();
            let user: user = serde_json::from_str(&injson.to_string())?;
            insert(user);
            //println!("{}",emp.emp_name);
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(injson.dump()))
        })
        .responder()
}