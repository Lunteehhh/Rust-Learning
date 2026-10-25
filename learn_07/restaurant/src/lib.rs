mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        
        pub fn serve_order() {}

        fn take_payment() {}
    }

}
use front_of_house::serving::{self, take_order} ;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    
    front_of_house::hosting::add_to_waitlist();
}
