extern crate chrono;

use reqwest::{self, Result};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct LoginRes {
    did: String,
    handle: String,
    email: String,
    accessJwt: String,
    refreshJwt: String
}

#[derive(Serialize, Deserialize, Debug)]
struct InviteCode {
    useCount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct InviteCodeRes {
    code: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Create {
    handle: String,
    email: String,
    password: String,
    inviteCode:String
}

#[derive(Serialize,Deserialize,Debug)]
struct CreateRes {
    handle: String,
    did: String,
    accessJwt: String,
    refreshJwt:String
}

#[derive(Serialize,Deserialize,Debug)]
struct PostRes {
    uri: String,
    cid: String
}

#[derive(Serialize,Deserialize,Debug)]
struct Post {
    repo: String,
    collection:String,
    record: Record
}
#[derive(Serialize,Deserialize,Debug)]
struct Record {
    _type: String,
    text: String,
    createdAt: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Login {
    identifier: String,
    password: String
}
pub struct ATP{
    base_url: String,
    pub identifier: String,
    did: String,
    jwt: String
}

impl Default for ATP {
    fn default() -> ATP {
        ATP {
            base_url: String::from("https://bsky.social/"),
            identifier: String::from(""),
            did: String::from(""),
            jwt: String::from("")
        }
    }
}

impl ATP {
    pub fn new(base_url:&String) -> ATP {
        ATP {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }

    pub fn create_invite_code(mut self, admin_username: String, admin_password: String, use_count: u32) -> Result<String> {
        let body = InviteCode {
            useCount: use_count,
        };
        let url = "".to_string()+&self.base_url+"/xrpc"+"com.atproto.server.createInviteCode";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .basic_auth(admin_username, Some(admin_password))
            .send()
            .unwrap();

        let res_json:InviteCodeRes;
        match res.json::<InviteCodeRes>() {
            Ok(json) => {
                res_json = json;
            },
            Err(e) => {
                return Err(e);
            }
        }
        Ok(res_json.code)
    }

    pub fn create_account(mut self, identifier:String, password:String, email:String, inviteCode:String) -> Result<String>{
        let body = Create {
            handle: identifier,
            email: email,
            password: password,
            inviteCode: inviteCode
        };
        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.server.createAccount";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .unwrap();
        let res_json: CreateRes;
        match res.json::<CreateRes>() {
            Ok(json) => {
                res_json = json;
            }
            Err(e) => {
                println!("Response was not ok: {}", e);
                return Err(e);
            }
        }
        self.jwt = res_json.accessJwt;
        self.did = res_json.did;
        Ok("".to_string())
    }

    pub fn login(&mut self,identifier: &String, password: String) -> Result<String> {
        let body = Login {
            identifier: identifier.to_owned(),
            password: password
        };
        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.server.createSession";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .unwrap();
        let res_json: LoginRes;
        match res.json::<LoginRes>() {
            Ok(json) => {
                res_json = json;
            }
            Err(e) => {
                println!("Response was not ok: {}", e);
                return Err(e);
            }
        }
        self.jwt = res_json.accessJwt;
        self.did = res_json.did;
        Ok("".to_string())
        
    }

    pub fn post(mut self, postText: String) -> Result<String> {
        let now = Utc::now().to_rfc3339().to_string();
        let body = Post {
            repo: self.did,
            collection: "app.bsky.feed.post".to_string(),
            record: Record {
                _type: "app.bsky.feed.post".to_string(),
                text: postText,
                createdAt: now
            }
        };

        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.repo.createRecord";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.jwt)
            .json(&body)
            .send()
            .unwrap();
        let res_json: PostRes;
        match res.json::<PostRes>() {
            Ok(json) => {
                res_json = json;
            }
            Err(e) => {
                println!("Response was not ok: {}", e);
                return Err(e);
            }
        }
        
        Ok("".to_string())
    }
}