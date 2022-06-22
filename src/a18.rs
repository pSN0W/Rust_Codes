// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

#[derive(Debug)]
struct Person{
    name : String,
    age : i32,
}

// we gave unit type to Ok of result since we are not returning anything significant
fn allowed(person:Person) -> Result<(),String> {
    if person.age < 21 {
        return Err("Age is less than 21".to_owned());
    }
    return Ok(());
}

fn main() {
    let person = Person {name : "John".to_owned(),age:22};
    let purchased = allowed(person);
    println!("{:?} ",purchased);
}
