use chrono::{NaiveDateTime, TimeDelta, };

use self::{combo::Combo, food::Food};

pub mod food;
pub mod combo;

#[derive(Debug)]
pub struct FoodController {
    pub combos: Vec<Combo>,
    pub foods: Vec<Food>,
    pub inventory: Vec<(
        String, //  food_name
        usize,  //  food_cnt_made
        usize,  //  food_cnt_leftover
        Option<NaiveDateTime>, //  latest_food_made_complete_time
    )>,
}

impl FoodController {
    pub fn new(foods: Vec<Food>, combos: Vec<Combo>) -> Self {
        FoodController {
            combos,
            foods,
            inventory: vec![]
        }
    }

    pub fn get_foods(food_name_list_str: &str, food_making_time_str: &str, food_max_capacity_str: &str) -> Vec<Food> {
        Food::get_foods(food_name_list_str, food_making_time_str, food_max_capacity_str)
    }

    pub fn get_combos(combo_list: &[&str], foods: &Vec<Food>) -> Vec<Combo> {
        Combo::get_combos(combo_list, foods)
    }

    pub fn pre_handle_inventory(&mut self, start_time: NaiveDateTime) {
        self.foods.iter().for_each(|x| {
            self.inventory.push((x.name.to_string(), 0, 0, Some(start_time)));
        })
    }

    pub fn making_food(&mut self) -> Option<(String, NaiveDateTime)> {
        let mut pre_next_food_completed: Vec<(String, NaiveDateTime)> = vec![];
        self.inventory.iter_mut().for_each(|food_inventory| {
            if let Some(food) = self.foods.iter().find(|x| x.name == food_inventory.0) {
                if food_inventory.2 < food.max_capacity {
                    let latest_food_made_time = food_inventory.3.unwrap();
                    let next_food_completed_time = latest_food_made_time.checked_add_signed(TimeDelta::seconds(food.time_of_making as i64)).unwrap();
                    pre_next_food_completed.push((food.name.clone(), next_food_completed_time));
                }
            };
        });
        let mut next_food: Option<(String, NaiveDateTime)> = None;
        pre_next_food_completed.iter().enumerate().for_each(|(i, (food_name, complete_time))| {
            if i == 0 {
                next_food = Some((food_name.to_owned(), complete_time.to_owned()));
            } else {
                if next_food.as_ref().unwrap().1 > *complete_time {
                    next_food = Some((food_name.to_owned(), complete_time.to_owned()));
                }
            }
        });
        next_food
    }
}