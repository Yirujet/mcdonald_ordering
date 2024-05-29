use super::food::Food;

#[derive(Debug)]
pub struct Combo {
    pub name: String,
    pub foods: Vec<(Food, u8)>
}

impl PartialEq for Combo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Combo {
    pub fn new(name: String) -> Self {
        Combo {
            name,
            foods: vec![]
        }
    }

    pub fn add_food(&mut self, food_name: &str, foods: &Vec<Food>) {
        let food = foods.iter().find(|x| {
            x.name == food_name
        }).unwrap();
        if self.foods.iter().any(|x| x.0.name == food_name) {
            self.foods.iter_mut().find(|x| x.0.name == food_name).unwrap().1 += 1;
        } else {
            let food = Food::new(food.name.to_string(), food.time_of_making, food.max_capacity);
            self.foods.push((food, 1));
        }
    }

    pub fn get_combos(combo_list: &[&str], foods: &Vec<Food>) -> Vec<Combo> {
        let mut combos: Vec<Combo> = vec![];
        combo_list.iter().for_each(|combo_content_str| {
            let combo_content: Vec<&str> = combo_content_str.split(" ").collect();
            let (combo_name, foods_included) = combo_content.split_at(1);
            let mut combo = Combo::new(combo_name.get(0).unwrap().to_string());
            foods_included.iter().for_each(|food_name| {
                combo.add_food(food_name, foods);
            });
            combos.push(combo);
        });
        combos
    }
}