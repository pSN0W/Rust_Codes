// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id : i32,
    quantity : i32,
}
// we are borrowing variable and not transferring ownership
fn print_id (item : &GroceryItem) {
    println!("id : {:?}",item.id);
}
// we are borrowing variable and not transferring ownership
fn print_quantity (item : &GroceryItem) {
    println!("quantity : {:?}",item.quantity);
}

fn main() {
    // initially main function has ownership of the variable item
    // so it is responsibility of main function to do clean up this memory
    let item = GroceryItem {
        id : 15,
        quantity : 38,
    };
    // borrow item variable
    print_id(&item);
    // borrow item variable
    print_quantity(&item);
    // clear the variable item at the end of scope
}
