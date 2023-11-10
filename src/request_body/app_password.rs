use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct AppSpecificPassword {
    pub name: String
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct AppSpecificPasswordRes {
    pub handle: String,
    pub did: String,
    pub accessJwt: String,
    pub refreshJwt:String
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct AppPassword {
    pub name: String,
    pub password: String,
    pub createdAt: String
}
