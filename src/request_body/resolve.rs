use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandle {
    pub handle: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolveHandleRes {
    pub did: String
}