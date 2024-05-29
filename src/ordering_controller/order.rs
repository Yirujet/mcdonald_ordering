use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

use crate::food_controller::{combo::Combo, food::Food};

#[derive(Debug)]
pub struct OrderDetail {
    pub foods_in_ticket: Vec<(String, u8)>,
    pub combos_in_ticket: Vec<(String, u8)>,
    pub foods_included: Vec<(
        String, //  food_name
        u8, //  food_cnt_needed
        u8  //  food_cnt_owned
    )>
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Created,
    Making,
    Complete,
    Fail
}

#[derive(Debug)]
pub struct Order {
    pub ticket_id: String,
    pub create_time: NaiveDateTime,
    pub complete_time: Option<NaiveDateTime>,
    pub status: Status,
    pub detail: OrderDetail
}

#[derive(Debug)]
pub enum OrderType {
    Food,
    Combo
}

impl Order {
    pub fn new(time: String) -> Self {
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
        let create_time = NaiveDateTime::parse_from_str(&(today + time.as_str()), "%Y-%m-%d %H:%M:%S").unwrap();
        Order {
            ticket_id: Uuid::new_v4().to_string(),
            create_time,
            complete_time: None,
            status: Status::Created,
            detail: OrderDetail {
                foods_in_ticket: vec![],
                combos_in_ticket: vec![],
                foods_included: vec![]
            }
        }
    }

    pub fn add_food(&mut self, food_name: &str, foods: &Vec<Food>) {
        if self.detail.foods_in_ticket.iter().any(|x| x.0 == food_name) {
            self.detail.foods_in_ticket.iter_mut().find(|x| x.0 == food_name).unwrap().1 += 1;
        } else {
            if let Some(food) = foods.iter().find(|x| {
                x.name == food_name
            }) {
                self.detail.foods_in_ticket.push((food.name.to_string(), 1));
            };
        }
    }

    pub fn add_combo(&mut self, combo_name: &str, combos: &Vec<Combo>) {
        if self.detail.combos_in_ticket.iter().any(|x| x.0 == combo_name) {
            self.detail.combos_in_ticket.iter_mut().find(|x| x.0 == combo_name).unwrap().1 += 1;
        } else {
            if let Some(combo) = combos.iter().find(|c| {
                c.name == combo_name
            }) {
                self.detail.combos_in_ticket.push((combo.name.to_string(), 1));
            }
        }
    }

    pub fn check_type(name: &str, foods: &Vec<Food>, combos: &Vec<Combo>) -> Option<OrderType>  {
        if foods.iter().any(|x| x.name == name) {
            Some(OrderType::Food)
        } else if combos.iter().any(|x| x.name == name) {
            Some(OrderType::Combo)
        } else {
            None
        }
    }

    pub fn calc_foods_included(&mut self, order_type: OrderType, name: &str, combos: &Vec<Combo>) {
        match order_type {
            OrderType::Combo => {
                if let Some(combo) = combos.iter().find(|c| {
                    c.name == name
                }) {
                    combo.foods.iter().for_each(|(food, food_cnt)| {
                        if self.detail.foods_included.iter().any(|x| x.0 == food.name) {
                            self.detail.foods_included.iter_mut().find(|x| x.0 == food.name).unwrap().1 += food_cnt;
                        } else {
                            self.detail.foods_included.push((food.name.to_string(), food_cnt.to_owned(), 0));
                        }
                    })
                }
            }
            OrderType::Food => {
                if self.detail.foods_included.iter().any(|x| x.0 == name) {
                    self.detail.foods_included.iter_mut().find(|x| x.0 == name).unwrap().1 += 1;
                } else {
                    self.detail.foods_included.push((name.to_string(), 1, 0));
                }
            }
        }
    }

    pub fn pre_handle_orders(order_rows: &[&str], foods: &Vec<Food>, combos: &Vec<Combo>) -> Vec<Order> {
        let mut orders: Vec<Order> = vec![];
        order_rows.iter().for_each(|o| {
            let order_list: Vec<&str> = o.split(" ").collect();
            let (create_time, order_content) = order_list.split_at(1);
            let mut order = Order::new(create_time.get(0).unwrap().to_string());
            order_content.iter().for_each(|name| {
                if let Some(order_type) = Order::check_type(name, foods, combos) {
                    match order_type {
                        OrderType::Combo => {
                            Order::add_combo(&mut order, name, combos);
                            Order::calc_foods_included(&mut order, order_type, name, combos);
                        }
                        OrderType::Food => {
                            Order::add_food(&mut order, name, foods);
                            Order::calc_foods_included(&mut order, order_type, name, combos);
                        }
                    }
                }
            });
            orders.push(order);
        });
        orders
    }
}