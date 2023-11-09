mod request_body;

use request_body::{
    record::Record,
    post::{Post, PostRes},
    login::{Login,LoginRes},
    invite::{InviteCode,InviteCodeRes},
    account_create::{CreateAccount,CreateAccountRes},
    refresh_session::RefreshSessionRes,
    resolve::{ResolveHandle,ResolveHandleRes}
};
use reqwest;
use chrono::prelude::*;
use derive_getters::Getters;

#[derive(Getters)]
pub struct ATP{
    base_url: String,
    pub identifier: String,
    did: String,
    jwt: String,
    refresh_jwt: String
}

impl Default for ATP {
    fn default() -> Self {
        Self {
            base_url: String::from("https://bsky.social/"),
            identifier: String::from(""),
            did: String::from(""),
            jwt: String::from(""),
            refresh_jwt: String::from("")
        }
    }
}

impl ATP {
    pub fn new(base_url:&String) -> ATP {
        Self {
            base_url: base_url.to_string(),
            ..Default::default()
        }
    }

    pub fn create_invite_code(self, admin_username: String, admin_password: String, use_count: u32) -> reqwest::Result<InviteCodeRes> {
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
        Ok(res_json)
    }

    pub fn create_account(mut self, identifier:String, password:String, email:String, invite_code:String) -> reqwest::Result<CreateAccountRes>{
        let body = CreateAccount {
            handle: identifier,
            email: email,
            password: password,
            inviteCode: invite_code
        };
        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.server.createAccount";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .unwrap();
        let res_json: CreateAccountRes;
        match res.json::<CreateAccountRes>() {
            Ok(json) => {
                res_json = json;
            }
            Err(e) => {
                println!("Account Creation Error: {}", e);
                return Err(e);
            }
        }
        self.jwt = res_json.accessJwt.to_owned();
        self.did = res_json.did.to_owned();
        Ok(res_json)
    }

    pub fn login(&mut self,identifier: &String, password: String) -> reqwest::Result<LoginRes> {
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
                println!("Login Error: {}", e);
                return Err(e);
            }
        }
        self.jwt = res_json.accessJwt.to_owned();
        self.refresh_jwt = res_json.refreshJwt.to_owned();
        self.did = res_json.did.to_owned();
        Ok(res_json)
        
    }

    pub fn refresh(&mut self) -> reqwest::Result<RefreshSessionRes>{
        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.server.refreshSession";

        let res = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type","application/json")
            .bearer_auth(self.jwt())
            .send()
            .unwrap();
        let res_json: RefreshSessionRes;

        match res.json::<RefreshSessionRes>() {
            Ok(json) => {
                res_json = json;
            },
            Err(e) => {
                return Err(e);
            }
        }
        self.jwt = res_json.accessJwt.to_owned();
        Ok(res_json)

    }
   
    pub fn post(&self, did: String, jwt: String, post_text: String) -> reqwest::Result<PostRes> {
        let now = Utc::now().to_rfc3339().to_string();
        let body = Post {
            repo: did,
            collection: "app.bsky.feed.post".to_string(),
            record: Record {
                _type: "app.bsky.feed.post".to_string(),
                text: post_text.to_owned(),
                createdAt: now
            }
        };

        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.repo.createRecord";

        let request = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(jwt)
            .json(&body)
            .send();

        let res: reqwest::blocking::Response;
        match request {
            Ok(response) => res = response,
            Err(e) => return Err(e)
        }

        let _res_json: PostRes;
        match res.json::<PostRes>() {
            Ok(json) => {
                Ok(json)
            }
            Err(e) => {
                println!("Post Error: {}", e);
                Err(e)
            }
        }
    }

    pub fn resolvehandle(&self, handle: String) -> reqwest::Result<String> {
        let body = ResolveHandle {
            handle: handle
        };

        let url = "".to_string()+&self.base_url+"xrpc/"+"com.atproto.identity.resolveHandle";

        let request = reqwest::blocking::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send();

        let res: reqwest::blocking::Response;
        match request {
            Ok(req) => res = req,
            Err(e) => return Err(e)
        }
        

        match res.json::<ResolveHandleRes>() {
            Ok(json) => Ok(json.did),
            Err(e) => return Err(e)
        }
        // Ok(res_json.did)
    }
}