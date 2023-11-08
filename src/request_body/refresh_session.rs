use serde::{Deserialize,Serialize};
use derive_getters::Getters;

#[derive(Serialize,Deserialize,Getters,Debug)]
#[allow(non_snake_case)]
pub struct RefreshSessionRes {
    pub accessJwt: String,
    pub refreshJwt: String,
    pub handle: String,
    pub did: String,
}