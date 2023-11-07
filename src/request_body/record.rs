use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Record {
    pub _type: String,
    pub text: String,
    pub createdAt: String
}
