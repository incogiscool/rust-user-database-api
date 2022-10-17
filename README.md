# rust-user-database-api

A simple REST API which stores users on a local store.json file.

Run `cargo run` in the terminal to get started.

**MAKE SURE THERE IS AT LEAST ONE STRUCT WITH `UserDataInput` TYPE (in JSON format (serialized))**
_One struct is included for you. If you would like, change up the values, as long as the keys are the same._

To **get** a user: send a `GET` request to your server (automatically is localhost:8000) to:
`/user/find/<user_id>`

To **upload** a user: send a `POST` request to your server (automatically is localhost:8000) to:
`/user/create/<user_id>`
with this format:

```
{
    first_name: String,
    last_name: String,
    email: String,
}
```

It should return an ID that you can use to reference back to the user.

To **delete** a user: send a `GET` request to your server (automatically is localhost:8000) to:
`/user/delete/<user_id>`

This was a personal project made by me for learning Rust and Rocket.rs. For any problems, suggestions, or improvements, please let me know via below.

```
Github: @incogiscool
Discord: incog#2102
Instagram: (@)ada.m.8
```
