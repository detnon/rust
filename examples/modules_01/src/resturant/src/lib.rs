mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
 
mod back_of_house {
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}

}
pub fn eat_at_resturaurant() {
    // Order breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change what bread we want
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This next line wont compile as we're not allowed to see
    // or modify the fruits
    // meal.seasonal_fruit = String::from("blueberries");


    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order(){}