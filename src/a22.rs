

fn main() {
    let a : Option<i32> = Some(1); // declaring an option variable
    let a_is_some = a.is_some(); // boolean representing if a is Some()
    let a_is_none = a.is_none(); // boolean denoting is a is None()
    let a_mapped = a.map(|num| num+1); // perform some functionality is a is Some
    let a_filtered = a.filter(|num| num==&1); // converts Some to None if answer of closure is false
                                              // borrows the value hence we use &1
    let a_or_else = a.or_else(|| Some(5)); // If a is None then replace it with Some(5)
    let unwrapped = a.unwrap_or_else(|| 0); // If a is some return value inside Some else return 0
}
