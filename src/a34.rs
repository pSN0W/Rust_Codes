// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state


#[derive(Debug)]
struct Luggage<T> {
    id: String,
    state: T
}

impl<T> Luggage<T> {
    fn transition<U> (self,nextState:U) -> Luggage<U> {
        Luggage {
            id: self.id,
            state: nextState
        }
    }
}

struct Packed;

impl Luggage<Packed> {
    fn new(id:&str) -> Self {
        Self {
            id:id.to_string(),
            state: Packed,
        }
    }

    fn checkin(self) -> Luggage<CheckIn> {
        self.transition(CheckIn)
    }
}

struct CheckIn;
impl Luggage<CheckIn> {
    fn load(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}

struct OnLoading;
impl Luggage<OnLoading> {
    fn drop(self) -> Luggage<Offloading>{
        self.transition(Offloading)
    }
}

struct Offloading;
impl Luggage<Offloading> {
    fn wait(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}

struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn pickup(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}

struct EndCustody;
impl Luggage<EndCustody> {
    fn removeLuggage(self) {
        println!("Luggage succesfully delivered");
    }
}


fn main() {
    let my_luggage = Luggage::<Packed>::new("ax4321");
    my_luggage.checkin().load().drop().wait().pickup().removeLuggage();
}
