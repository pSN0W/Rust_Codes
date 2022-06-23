// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter


struct Square{
    dimension:i32
}

struct Triangle{
    a:i32,
    b:i32,
    c:i32,
}
// Any struct being part of this trait must implement all the functions described 
// in the trait
trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

// implementing this trait for the square
impl Perimeter for Square{
    fn calculate_perimeter(&self) -> i32 {
        4*self.dimension
    }
}

// Implementing the trait for Triangle
impl Perimeter for Triangle{
    fn calculate_perimeter(&self) -> i32 {
        self.a + self.b + self.c
    }
}

// This function argument is anything that implements Perimeter
// That means this function can be called on anything that implements Perimeter
fn perimeter(thing:impl Perimeter) {
    let per = thing.calculate_perimeter();
    println!("Perimeter is {:?}",per);
}
fn main() {
    perimeter(Square{dimension:4});
    perimeter(Triangle{a:1,b:2,c:3});
}
