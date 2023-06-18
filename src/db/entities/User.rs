use chrono::DateTime;

struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password: String,

    created_by: String,
    created_on: String,
    modified_by: String,
    modified_on: String,
}