use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct Record {
    pub _type: String,
    pub text: String,
    pub createdAt: String
}
