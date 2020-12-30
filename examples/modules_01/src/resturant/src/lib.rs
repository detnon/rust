mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
 
mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

}
pub fn eat_at_resturaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order(){}
