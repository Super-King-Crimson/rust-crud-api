use super::*;

pub fn handle_post_request(req: &str) -> (String, String) {
    match (parse_user_from_req(req), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client.execute(
                "INSERT INTO users (name, email) VALUES ($1, $2)",
                &[&user.name, &user.email]
            ).unwrap();

            (HTTP_OK.to_string(), String::from("User created"))
        }
        _ => (HTTP_SERVER_ERROR.to_string(), String::from("Error")),
    }
}

pub fn handle_get_request(req: &str) -> (String, String) {
    match (get_id(req).parse::<u32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User::new(row.get(0), row.get(1), row.get(2));

                    (HTTP_OK.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (HTTP_NOT_FOUND.to_string(), String::from("User not found")),
            }
        }
        _ => (HTTP_SERVER_ERROR.to_string(), String::from("Error"))
    }
}

pub fn handle_get_all_request(_req: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let rows = client.query("SELECT * FROM users", &[]).unwrap();
            let users: Vec<User> = rows.iter()
                .map(|row| {
                    User::new(row.get(0), row.get(1), row.get(2))
                })
                    .collect();

            (HTTP_OK.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (HTTP_SERVER_ERROR.to_string(), String::from("Error"))
    }
}

pub fn handle_put_request(req: &str) -> (String, String) {
    match (
        get_id(req).parse::<i32>(),
        parse_user_from_req(req),
        Client::connect(DB_URL, NoTls)
    ) 
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client.execute(
                "UPDATE users SET name = $1, email = $2 WHERE id = $3", 
                &[&user.name, &user.email, &id]
            ).unwrap();

            (HTTP_OK.to_string(), String::from("User updated"))
        }
        _ => (HTTP_SERVER_ERROR.to_string(), String::from("Error"))
    }
}

pub fn handle_delete_request(req: &str) -> (String, String) {
    match (
        get_id(req).parse::<u32>(),
        Client::connect(DB_URL, NoTls)
    ) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute(
                "DELETE FROM users WHERE id = $1", 
                &[&id],
            ).unwrap();

            if rows_affected == 0 { return (HTTP_NOT_FOUND.to_string(), String::from("User not found")) }

            (HTTP_OK.to_string(), String::from("Deleted user"))
        }
        _ => (HTTP_SERVER_ERROR.to_string(), String::from("Error"))
    }
}
