use std::env;
extern crate dotenv;

use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODBURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading .env"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: user.name,
            last_name: user.last_name,
            country: user.country,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("An error occurred while creating the user");

        Ok(user)
    }

    pub fn get_user_by_id(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("An error occurred while get the user");
        Ok(user_detail.unwrap())
    }

    // pub fn get_user_by_name(&self, name: &String) -> Result<User, Error> {
    //     let obj_name = ObjectId::parse_str(name).unwrap();
    //     let filter = doc! {"name": obj_name};
    //     let user_detail = self
    //         .col
    //         .find_one(filter, None)
    //         .ok()
    //         .expect("An error occurred while get the user by name");
    //     Ok(user_detail.unwrap())
    // }

    pub fn update_user(&self, id: &String, user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": user.id,
                    "name": user.name,
                    "last_name": user.last_name,
                    "country": user.country
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("An error occurred while updating the user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("An error occurred while deleting the user");

        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("An error occurred while getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}
