use actix_web::{
    HttpRequest, Responder,
};
use cdrs::query::*;
use cdrs::types::rows::Row;
use dbconnection;
use users::user::user;
use serde_json;

use cdrs::{self, types::prelude::*};
use cdrs::frame::TryFromRow;

fn view() -> Vec<Row>
{
    let session = dbconnection::connection();

    let select_struct_cql = "Select * from auction.users";
    let row = session
        .query(select_struct_cql)
        .expect("query")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into row");

    return row;
}

pub fn get_manual(req: &HttpRequest) -> impl Responder
{
    let rows = view();
//println!("{}",rows.len());
    let mut my_row: user = user {
        id: String::new(),
        name: String::new(),
    };


    let mut allusers: String;
    for row in rows {
        my_row = user::try_from_row(row).expect("into Item Struct");
        println!("Struct got :{:?}", my_row);
        //allusers = allusers + my_row.to_string();
    }


    let jsonstring = serde_json::to_string(&my_row);

    return jsonstring;
}
