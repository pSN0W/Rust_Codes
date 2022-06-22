// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    numbers.push(40);
    // we need to use borrowing here also since it has a scope
    for num in &numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}",num)
        };
    }
    println!("Number of elements in vec is {:?}",numbers.len());
}
