// fn is_even(num: i32)-> bool {
//     if num % 2 == 0 {
//         return true;
//     }
//     return false;
// }

// fn main() {
//     println!("{}", is_even(50));
// }

// fn fibonacci(num: i32) -> i32 {
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }

//     if num == 1 {
//         return second;
//     }

//     for _ in 0..(num - 1){
//         let temp = second;
//         second = second + first;
//         first = temp
//     }

//     return second;
// }

// fn main() {
//     println!("{}", fibonacci(4));
// }

// fn get_str_len(s: &str) -> usize {
//     s.chars().count()
// }

// fn main() {
//     let my_string = String::from("Hello world!!");
//     println!("{}", get_str_len(&my_string));
// }

//struct

// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }

// fn main() {
//     let user = User{
//         first_name: String::from("Sada"),
//         last_name: String::from("shiva"),
//         age: 23,
//     };

//     println!("User details, firstName: {}, lastName: {}, age: {}", user.first_name, user.last_name, user.age);
// }

// struct Rect {
//     height: u32,
//     width: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }

// fn main() {
//     let rect = Rect {
//         height: 3,
//         width: 6
//     };
        
//     println!("Area of rect is: {}", rect.area());
// }

// enum Shape {
//     Rectangle(f64, f64),
//     Circle(f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape{
//         Shape::Rectangle(l, b) => l * b,
//         Shape::Circle(r) => 3.14 * r * r,
//     }
// }

// fn main() {
//     let rect = Shape::Rectangle(3.0, 6.0);
//     println!("Area of Reactangle is: {}", calculate_area(rect));

//     let circle = Shape::Circle(5.0);
//     println!("Area of circle is: {}", calculate_area(circle));
// }



// //enum with options

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }

//     return None;
// }

// fn main() {
//     let index = find_first_a(String::from("skjvanc"));
    
//     match index {
//         Some(value) => println!("'a' found at index: {}", value),
//         None => println!("'a' not found in the given string."),
//     }
// }



// // enum with result

// use std::fs;        // importing fs library(read_to_string is from fs library)

// fn main() {
//     let file_content = fs::read_to_string("src/a.txt");

//     match file_content {
//         Ok(value) => {
//             println!("{}", value);
//         },
//         Err(error) => {
//             println!("{}", error);
//         },
//     }
// }



// package manager(we use cargo in rust to install packages eg, here we used 'cargo add chrono' which is says time)

use chrono::{self, Local, Utc};
fn main() {
    let current_time = Local::now();
    println!("Current time is: {}", current_time);

    println!("Current time in UTC: {}", Utc::now());
}