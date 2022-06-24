// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle{
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialOrd, PartialEq)]
enum Status{
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental{
    vehicle : Vehicle,
    status : Status,
    vin : String,
}

#[derive(Debug)]
struct Corporate{
    rental : Rc<RefCell<Vec<Rental>>>
}

#[derive(Debug)]
struct StoreFront{
    rental : Rc<RefCell<Vec<Rental>>>
}

fn main() {
    let vehicles = vec![
        Rental{
            vehicle : Vehicle::Car,
            status : Status::Maintenance,
            vin : "1".to_owned()
        },
        Rental{
            vehicle : Vehicle::Truck,
            status : Status::Available,
            vin : "2".to_owned()
        },
    ];
    println!("Initially -> ");
    dbg!(&vehicles);
    let vehicles = Rc::new(RefCell::new(vehicles));
    let corporate = Corporate{rental:Rc::clone(&vehicles)};
    let store_front = StoreFront{rental:Rc::clone(&vehicles)};
    {
        let mut rental = store_front.rental.borrow_mut();
        if let Some(vehicle) = rental.get_mut(1) {
            println!("Vehicle {} rented",vehicle.vin);
            vehicle.status = Status::Rented;
        }
    }
    {
        let ren = vehicles.borrow();
        dbg!(&ren);
    }
    {
        let mut rental = corporate.rental.borrow_mut();
        if let Some(vehicle) = rental.get_mut(0) {
            println!("Vehicle {} is available",vehicle.vin);
            vehicle.status = Status::Available;
        }
    }
    {
        let ren = vehicles.borrow();
        dbg!(&ren);
    }
}
