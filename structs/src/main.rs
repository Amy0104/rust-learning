#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("tom@example.com"),
        name: String::from("tom"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("lucy@example.com"),
        ..user1
    };

    println!("{:#?}", user2);
}

// fn build_user(emial: String, name: String) -> User {
//     User {
//         email,
//         name,
//         active: true,
//         sign_in_count: 1,
//     }
// }
