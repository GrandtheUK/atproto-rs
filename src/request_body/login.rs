use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LoginRes {
    pub did: String,
    pub handle: String,
    pub email: String,
    pub accessJwt: String,
    pub refreshJwt: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub identifier: String,
    pub password: String
}
