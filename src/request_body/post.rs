use super::record::Record;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct PostRes {
    pub uri: String,
    pub cid: String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Post {
    pub repo: String,
    pub collection:String,
    pub record: Record
}