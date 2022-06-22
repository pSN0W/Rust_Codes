// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn generate_tuple () -> (i32,i32) {
    (1,2)
}

fn main() {
    let (_,y) = generate_tuple();
    if y>5 {
        println!("greater then 5");
    } else if y==5 {
        println!("equal to 5");
    } else {
        println!("less than 5");
    }
}
