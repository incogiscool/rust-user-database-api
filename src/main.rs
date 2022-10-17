extern crate rocket;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use colored::Colorize;
use rocket::{
    delete, get, launch, post, routes,
    serde::{
        json::{from_str, serde_json::to_string_pretty, Json},
        Deserialize, Serialize,
    },
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
struct User {
    first_name: String,
    last_name: String,
    email: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
struct UserDataInput {
    enter_timestamp: u128,
    uid: String,
    payload: User,
}

fn get_and_store(file: User) -> String {
    let time: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error creating timestamp")
        .as_millis();

    let user_id = Uuid::new_v4().to_string();

    let user1_data = UserDataInput {
        uid: user_id.clone(),
        enter_timestamp: time,
        payload: file,
    };

    let file_json_path: &str = "store.json";

    let mut contents_deserialized = get_store_and_deserialize(file_json_path);
    contents_deserialized.push(user1_data);

    let contents_serialized: String = to_string_pretty(&contents_deserialized).unwrap();

    let mut file_write: File = OpenOptions::new().write(true).open(file_json_path).unwrap();
    let data_serialized_buf: &[u8] = contents_serialized.as_bytes();

    file_write
        .write_all(data_serialized_buf)
        .expect("Could not write to file.");

    println!("Data Written...");

    return user_id;
}

fn get_store_and_deserialize(file_path: &str) -> Vec<UserDataInput> {
    let mut file_contents = String::new();
    let mut file_read: File = File::open(file_path).expect("Could not open file.");

    file_read
        .read_to_string(&mut file_contents)
        .expect("Could not read file.");

    let contents_deserialized: Vec<UserDataInput> = from_str(file_contents.as_str()).unwrap();

    return contents_deserialized;
}

fn get_user_by_id(user_id: String) -> Result<UserDataInput, String> {
    let file_json_path: &str = "store.json";
    let db: Vec<UserDataInput> = get_store_and_deserialize(file_json_path);

    for user in db {
        if user.uid == user_id {
            return Ok(user);
        }
    }

    return Err("User not found.".to_string());
}

#[post("/user/create", format = "json", data = "<input>")]
fn post_test(input: Json<User>) -> String {
    let Json(input) = input;

    println!("{:?}", input);
    let user_id = get_and_store(input);
    println!("-----------------------------------------------------");
    println!("Stored with UID: {}", user_id.as_str().bold().red());
    println!("-----------------------------------------------------");

    format!("{}", user_id)
}

#[get("/user/find/<user_id>")]
fn get_user(user_id: &str) -> String {
    let res = get_user_by_id(user_id.to_string());

    match res {
        Ok(user) => {
            let serialized = to_string_pretty(&user).unwrap();

            return serialized;
        }
        Err(error) => return error,
    }
}

#[delete("/user/delete/<user_id>")]
fn delete_user(user_id: &str) -> String {
    let file_json_path: &str = "store.json";

    let mut db = get_store_and_deserialize(file_json_path);
    let res = get_user_by_id(user_id.to_string());

    match res {
        Ok(user) => {
            let mut db_iter = db.iter();
            let index = db_iter.position(|user_iter| *user_iter == user).unwrap();

            db.remove(index);
            let serialized = to_string_pretty(&db).unwrap();

            let mut file_write: File = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_json_path)
                .unwrap();
            let data_serialized_buf: &[u8] = serialized.as_bytes();

            file_write
                .write_all(data_serialized_buf)
                .expect("Could not write to file.");

            println!("Data deleted...");

            return "User deleted.".to_string();
        }
        Err(error) => return error,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![post_test, get_user, delete_user])
}
