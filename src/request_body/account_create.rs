use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct CreateAccount {
    pub handle: String,
    pub email: String,
    pub password: String,
    pub inviteCode: String
}

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct CreateAccountRes {
    pub handle: String,
    pub did: String,
    pub accessJwt: String,
    pub refreshJwt:String
}

