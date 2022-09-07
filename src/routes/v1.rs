use crate::{models::user_model::User, controllers::mongodb_controller::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<id>")]
pub fn get_user_by_id(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user_by_id(&id);

    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[get("/user/name/<name>")]
// pub fn get_user_by_id(db: &State<MongoRepo>, name: String) -> Result<Json<User>, Status> {
//     if name.is_empty() {
//         return Err(Status::BadRequest);
//     };
//     let user_detail = db.get_user_by_name(&name);

//     match user_detail {
//         Ok(user) => Ok(Json(user)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

#[post("/user", data = "<user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: user.name.to_owned(),
        last_name: user.last_name.to_owned(),
        country: user.country.to_owned(),
    };

    let user_detail = db.create_user(data);

    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<id>", data = "<user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    id: String,
    user: Json<User>,
) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: user.name.to_owned(),
        last_name: user.last_name.to_owned(),
        country: user.country.to_owned(),
    };
    let update_result = db.update_user(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user_by_id(&id);

                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<id>")]
pub fn delete_user(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}