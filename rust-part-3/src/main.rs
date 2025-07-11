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

fn get_str_len(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Hello world!!");
    println!("{}", get_str_len(&my_string));
}
