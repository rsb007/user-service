
use cdrs::{self,types::prelude::*};
use std::str;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::frame::IntoBytes;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]

pub struct user {
    pub id: String,
    pub name: String,
}