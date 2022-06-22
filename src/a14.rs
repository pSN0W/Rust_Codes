// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name : String,
    age : i32,
    color : String,
}

fn print_name (name : &str) {
    println!("Name : {:?}",name);
}

fn print_color (color : &str) {
    println!("Color : {:?}",color);
}

fn main() {
    let persons = vec![
        Person {
            name:"Pratyaksh".to_owned(),
            age : 20,
            color : "Red".to_owned()
        },
        Person {
            name:String::from("Janhavi"),
           age :  5,
           color :  String::from("Pink")
        },
        Person {
            name:"Pratyush".to_owned(),
           age :  7,
           color :  String::from("Black")
        }
    ];

    for person in &persons {
        if person.age < 10 {
            print_name(&person.name);
            print_color(&person.color);
        }
    }
}
