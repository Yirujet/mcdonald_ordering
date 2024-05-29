use std::cmp::Ordering;

use chrono::{Local, NaiveDateTime, TimeDelta};

use crate::food_controller::FoodController;

use self::order::{Order, Status};

pub mod order;

#[derive(Debug)]
pub struct OrderingController {
    pub open: bool,
    pub judge_open_or_not: (u8, u8),
    pub business_time: (NaiveDateTime, NaiveDateTime),
    pub orders: Vec<Order>,
    pub current_time: Option<NaiveDateTime>,
    pub backlog_orders: Vec<String>,
    pub next_open_time: Option<NaiveDateTime>,
}

impl OrderingController {
    pub fn new(judge_open_or_not: (u8, u8), business_time: (NaiveDateTime, NaiveDateTime)) -> Self {
        OrderingController {
            open: true,
            judge_open_or_not,
            business_time,
            orders: vec![],
            current_time: None,
            backlog_orders: vec![],
            next_open_time: None,
        }
    }

    pub fn pre_handle_orders(&mut self, orders: &[&str], food_controller: &FoodController) {
        self.orders = Order::pre_handle_orders(orders, &food_controller.foods, &food_controller.combos);
    }

    pub fn check_out_fail_order(&mut self) {
        self.orders.iter_mut().for_each(|order| {
            if order.create_time < self.business_time.0 {
                order.status = Status::Fail;
            } else if order.create_time > self.business_time.1 {
                order.status = Status::Fail;
            }
        })
    }

    pub fn handle_open(&mut self, second_moving: (String, NaiveDateTime)) {
        self.backlog_orders = self.orders.iter().filter_map(|order| {
            match order.create_time.cmp(&second_moving.1) {
                Ordering::Less => {
                    match order.status {
                        Status::Making => {
                            Some(order.ticket_id.clone())
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }).collect();
        let (min_cnt, max_cnt) = self.judge_open_or_not;
        if self.backlog_orders.len() > max_cnt as usize {
            self.open = false;
        } else if self.backlog_orders.len() < min_cnt as usize {
            if self.open == false {
                if let Some(next_open_time) = self.next_open_time {
                    if second_moving.1 >= next_open_time {
                        self.open = true;
                    }
                } else {
                    self.next_open_time = second_moving.1.checked_add_signed(TimeDelta::seconds(1));
                }
            }
        }
    }

    pub fn handle_orders(&mut self, food_controller: &mut FoodController, next_completed_food_time: NaiveDateTime) {
        for order in self.orders.iter_mut() {
            match order.status {
                Status::Created | Status::Making => {
                    if order.create_time <= next_completed_food_time {
                        match order.status {
                            Status::Created => {
                                if self.open {
                                    order.status = Status::Making;
                                } else {
                                    order.status = Status::Fail;
                                    continue;
                                }
                            }
                            _ => {}
                        }
                        order.detail.foods_included.iter_mut().for_each(|order_food| {
                            let food_inventory = food_controller.inventory.iter_mut().find(|x| {
                                x.0 == order_food.0
                            }).unwrap();
                            if order_food.2 < order_food.1 {
                                if food_inventory.2 > (order_food.1 - order_food.2) as usize {
                                    food_inventory.2 -= (order_food.1 - order_food.2) as usize;
                                    order_food.2 = order_food.1;
                                } else {
                                    order_food.2 += food_inventory.2 as u8;
                                    food_inventory.2 = 0;
                                }
                            }
                        });
                    }
                    let completed = order.detail.foods_included.iter().all(|x| {
                        x.1 == x.2
                    });
                    if completed {
                        order.status = Status::Complete;
                        order.complete_time = Some(next_completed_food_time);
                        let today = Local::now().date_naive().format("%Y-%m-%d");
                        let end_time = NaiveDateTime::parse_from_str(&(today.to_string() + "23:59:59"), "%Y-%m-%d %H:%M:%S").unwrap();
                        if next_completed_food_time > end_time {
                            order.complete_time = Some(end_time);
                        }
                        if let Some(order_index) = self.backlog_orders.iter().position(|x| x == &order.ticket_id) {
                            self.backlog_orders.remove(order_index);
                        }
                    }
                }
                Status::Fail => {
                    order.complete_time = None;
                }
                Status::Complete => {
                }
            }
        }
    }
}