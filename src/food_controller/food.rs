#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Food {
    pub name: String,
    pub time_of_making: u8,
    pub max_capacity: usize,
}

impl Food {
    pub fn new(name: String, time_of_making: u8, max_capacity: usize) -> Self {
        Food {
            name,
            time_of_making,
            max_capacity,
        }
    }

    pub fn get_foods(food_name_list_str: &str, food_making_time_str: &str, food_max_capacity_str: &str) -> Vec<Food> {
        let food_name_list: Vec<&str> = food_name_list_str.split(" ").collect();
        let food_making_time_list: Vec<&str> = food_making_time_str.split(",").collect();
        let food_max_capacity_str: Vec<&str> = food_max_capacity_str.split(",").collect();
        let mut foods: Vec<Food> = vec![];
        food_name_list.iter().enumerate().for_each(|(i, name)| {
            let food = Food::new(
                name.to_string(), 
                food_making_time_list.get(i).unwrap().parse().unwrap(), 
                food_max_capacity_str.get(i).unwrap().parse().unwrap()
            );
            foods.push(food);
        });
        foods.push(Food::new(
            "second_moving".to_string(), 
            1, 
            86400
        ));
        foods
    }
}