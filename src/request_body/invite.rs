use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCode {
    pub useCount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteCodeRes {
    pub code: String,
}
