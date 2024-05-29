use std::{fs::{self, File}, io::Write, path::Path};

use chrono::{Local, NaiveDateTime};
use ordering_controller::order::Status;

use crate::{food_controller::FoodController, ordering_controller::OrderingController};

mod food_controller;
mod ordering_controller;

fn main() {
    let dict_content = fs::read_to_string("assets/dict.dic").unwrap();
    let input_content = fs::read_to_string("assets/input.txt").unwrap();
    let dict_rows: Vec<&str> = dict_content.lines().collect();
    let input_rows: Vec<&str> = input_content.lines().collect();
    let threshold: Vec<&str> = input_rows.get(1).unwrap().split(",").collect();
    let max_threshold = threshold.get(0).unwrap().parse().unwrap();
    let min_threshold = threshold.get(1).unwrap().parse().unwrap();

    let food_name_list_str = dict_rows.get(1).unwrap();
    let food_making_time_str = input_rows.get(2).unwrap();
    let food_max_capacity_str = input_rows.get(3).unwrap();
    let (_, combo_list) = dict_rows.split_at(2);

    let foods = FoodController::get_foods(food_name_list_str, food_making_time_str, food_max_capacity_str);
    let combos = FoodController::get_combos(combo_list, &foods);

    let mut food_controller = FoodController::new(foods, combos);

    let today = Local::now().date_naive().format("%Y-%m-%d");
    let business_start_time_str = "07:00:00";
    let business_end_time_str = "22:00:00";
    let business_start_time = NaiveDateTime::parse_from_str(&(today.to_string() + business_start_time_str), "%Y-%m-%d %H:%M:%S").unwrap();
    let business_end_time = NaiveDateTime::parse_from_str(&(today.to_string() + business_end_time_str), "%Y-%m-%d %H:%M:%S").unwrap();

    let mut ordering_controller = OrderingController::new(
        (min_threshold, max_threshold),
        (business_start_time, business_end_time)
    );
    let (_, orders) = input_rows.split_at(4);

    food_controller.pre_handle_inventory(ordering_controller.business_time.0);

    ordering_controller.pre_handle_orders(orders, &food_controller);
    ordering_controller.check_out_fail_order();

    loop {
        let next_completed_food = food_controller.making_food();
        match next_completed_food {
            Some(next_completed_food) => {
                if let Some(food) = food_controller.inventory.iter_mut().find(|food| {
                    food.0 == next_completed_food.0
                }) {
                    food.1 += 1;
                    food.2 += 1;
                    food.3 = Some(next_completed_food.1);
                }
                if next_completed_food.0 == "second_moving" {
                    ordering_controller.handle_open(next_completed_food.clone());
                }
                ordering_controller.handle_orders(&mut food_controller, next_completed_food.1);
            }
            None => {
                let output_file_path = Path::new("assets/output.txt");
                if let Ok(output_file_exist) = output_file_path.try_exists() {
                    if output_file_exist {
                        let _ = fs::remove_file(output_file_path);
                    }
                    if let Ok(mut f) = File::create("assets/output.txt") {
                        ordering_controller.orders.iter().for_each(|order| {
                            match order.status {
                                Status::Complete => {
                                    f.write_all(format!("【订单号】 - {}\n", order.ticket_id).as_bytes()).expect("write filed");
                                    f.write_all("【下单成功】\n".as_bytes()).expect("write filed");
                                    f.write_all(format!("【下单时间】 - {}\n", order.create_time).as_bytes()).expect("write filed");
                                    f.write_all(format!("【完成时间】 - {}\n", order.complete_time.unwrap()).as_bytes()).expect("write filed");
                                    f.write_all("【订单明细】\n".as_bytes()).expect("write filed");
                                    order.detail.foods_included.iter().for_each(|food| {
                                        f.write_all(format!("- {} * {}\n", food.0, food.1).as_bytes()).expect("write filed");
                                    });
                                    f.write_all("\n".as_bytes()).expect("write filed");
                                }
                                Status::Fail => {
                                    f.write_all(format!("【订单号】 - {}\n", order.ticket_id).as_bytes()).expect("write filed");
                                    f.write_all("【下单失败】\n".as_bytes()).expect("write filed");
                                    f.write_all(format!("【下单时间】 - {}\n", order.create_time).as_bytes()).expect("write filed");
                                    f.write_all("\n".as_bytes()).expect("write filed");
                                }
                                _ => {}
                            }
                        });
                    }
                }
                break;
            }
        }
    }
}
