struct User{
    email: String,
    username: String,
    active_count: u32
}
fn main() {
    let user1 = User{
        email: String::from("paritosh21w@gmail.com"),
        username: String::from("paritosh"),
        active_count: 1
    };
    println!("{}", user1.email);
    let user2 = make_user(String::from("sd@gamil.com"), String::from("sdf"));
    println!("{}", user2.username);
}

fn make_user(email:String, username:String)->User {
    User{email, username, active_count:1}
}
