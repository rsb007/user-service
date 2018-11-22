use dbconnection::*;
use cdrs::query::QueryExecutor;


pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE auction.users (
    id text,
    name text,
    PRIMARY KEY (id))";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

