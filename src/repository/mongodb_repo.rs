use std::env;
// extern crate dotenv;
// use dotenv::dotenv;


use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc}, //modify here
    results::{ InsertOneResult},
    Client, Collection,
};
use crate::models::user_model::User;

static db_uri: &'static str = "mongodb+srv://nikolajgt:Hm4p5m59@cluster0.0y5kbeb.mongodb.net/test";

pub struct MongoRepo {
    col: Collection<User>
}

//THE IMPL IS ON MONGO_REPO SO WHEN CALLING SELF, JUST MEANS THE COLLECTION AND IN THIS CASE ITS USER COLLECTION
impl MongoRepo {
    //THIS 
    pub async fn init() -> Self {
        // dotenv().ok();
        let client_result = Client::with_uri_str(db_uri).await;
        let client = match client_result {
            Ok(_o) => _o,
            Err(_Err) => panic!("Error at MongoRepo init: {}", _Err)
        };

        let db = client.database("rustdb");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }


    //CREATE USER
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name:new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
                    .col
                    .insert_one(new_doc, None)
                    .await
                    .ok()
                    .expect("Error at creation of user");
        
        Ok(user)
    }


    //GET USER
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
                    .col
                    .find_one(filter, None)
                    .await
                    .ok()
                    .expect("Eror at getting user´s details");
        
        Ok(user_detail.unwrap())
    }
}