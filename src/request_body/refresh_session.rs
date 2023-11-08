use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
#[allow(non_snake_case)]
pub struct RefreshSessionRes {
    pub accessJwt: String,
    pub refreshJwt: String,
    pub handle: String,
    pub did: String,
}