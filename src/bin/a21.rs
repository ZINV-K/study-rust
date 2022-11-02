// map combinator

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

impl User {
    fn new(user_id: i32, name: &str) -> Self {
        Self {
            user_id,
            name: name.to_string(),
        }
    }
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(3),
        "katie" => Some(9),
        _ => None,
    }
}
fn main() {
    let user_name = "sam";

    let user = find_user(user_name).map(|user_id| User::new(user_id, user_name));
    match user {
        Some(user) => println!("{:?}", user),
        None => println!("User not found"),
    }
}
