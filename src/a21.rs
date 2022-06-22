// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let name = "john".to_owned();
    // we can use map on options. 
    // If the option value is None then None is returned
    // Otherwise the closure inside map is applied on Some() value of option
    // we can chain map
    // closure =====> |arguments| return_value
    let user_exist = find_user(&name)
                            .map(|x| User{user_id:x,name:name})
                            .map(|user| println!("{:?}",user));

    // the result of all the map is option itself so 
    // we will have to deconstruct it                        
    match user_exist {
        None => println!("User does not exist"),
        _ => ()
    };
}
