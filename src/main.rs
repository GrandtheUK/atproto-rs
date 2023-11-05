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
    identifier: String,
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
    fn login(&mut self,identifier: String, password: String) -> Result<String> {
        let body = Login {
            identifier: identifier,
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

    fn post(mut self, postText: String) -> Result<String> {
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

fn main() {
    let mut atproto = ATP {
        base_url: "https://bluesky.benradford.me/".to_string(),
        ..Default::default()
    };
    println!("Provider: {}",atproto.base_url);
    let identity = "grand.bluesky.benradford.me".to_string();
    let password = "CW4#C8ot1S&VH*".to_string();
    match atproto.login(identity,password) {
        Ok(_) => {
            println!("Login Success");
            match atproto.post("This is a test message".to_string()) {
                Ok(_) => println!("Post made"),
                Err(_) => println!("Post could not be made")
            }
        },
        Err(_) => {
            println!("Login Failure. Please retry");
        }
    }

}
