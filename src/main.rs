//Hello world program

// fn main() {
//     println!("Hello, world!");
// }

//1. Numbers

// fn main() {
//     let x: i32 = 1;
//     println!("{}", x);
// }

//what happen if it overflows

// fn main() {
//     let mut num: i8 = 124;
//     for i in 0..100 {
//         num = num.wrapping_add(127);
//     }
//     print!("Number: {}", num);
// }

//2. Booleans

// fn main() {
//     let is_male = false;
//     let is_above_18 = true;
    
//     if is_male {
//         println!("You are a male");

//     } else {
//         println!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male");
//     }
// }

//3. Strings

// fn main() {
//     let greeting = String::from("hello world");

//     if let Some(nth_char) = greeting.chars().nth(4) {
//         println!("The 4th character is: {}", nth_char);
//     } else {
//         println!("There is no 4th character in the string.");
//     }
// }

//Conditionals

// pub fn main() {
//     let x = 99;
//     let is_even = is_even(x);
//     if is_even {
//         print!("{} is even", x);
//     } else {
//         print!("{} is odd", x);
//     }
// }

// pub fn is_even(x: i32) -> bool {
//     return x % 2 == 0;
// }

//Loops

// pub fn main() {
//     let str = String::from("harkirat singh");
//     println!("First name {}", get_first_name(str));
//     print!("sum {}", do_sum(123, 456));
    
// }

// pub fn get_first_name(str: String) -> String {
//     let mut first_name = String::from("");
//     for c in str.chars() {
//         if c == ' ' {
//             break
//         }
//         first_name.push(c);
//     }
//     return first_name;
// }

//Functions

// fn do_sum(a: i32, b: i32) -> i32 {
// 	return a + b;
// }

// fn main() {
//     stack_fn();   // Call the function that uses stack memory
//     heap_fn();    // Call the function that uses heap memory
//     update_string();  // Call the function that changes size of variable at runtime
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointers: {:p}", s.capacity(), s.len(), s.as_ptr());

//     // Append some text to the string
//     for _ in 0..100{
//     s.push_str(" and some additional text");
//     println!("After update: {}", s);
//     println!("Capacity: {}, Length: {}, Pointers: {:p}", s.capacity(), s.len(), s.as_ptr());
//     }
// }

//Heap variables

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s2); // This line would cause a compile error because ownership has been moved.
// }

// fn main() {
//     let mut my_string = String::from("hello");
//     // takes_ownership(my_string.clone());
//     my_string = takes_ownership(my_string);
//     println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
// }

// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string); // `some_string` now owns the data.
//     return some_string;
// }

//References

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = &s1;

//     println!("{}", s2);
//     println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
// }

//Borrowing

// fn main() {
//     let my_string = String::from("Hello, Rust!");
//     takes_ownership(&my_string);  // Pass a reference to my_string
//     println!("{}", my_string);    // This is valid because ownership was not transferred
// }

// fn takes_ownership(some_string: &String) {
//     println!("{}", some_string);  // some_string is borrowed and not moved
// }

//Mutable references

// fn main() {
//     let mut s1 = String::from("Hello");
//     update_word(&mut s1);
//     println!("{}", s1);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }


// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = &s1;
//     let s3 = &s1;
    
//     println!("{}", s1);
//     println!("{}", s2);
//     println!("{}", s3);
// }

// fn main() {
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;
//     let s3 = update_word(&mut s1);
    
//     println!("{}", s1);
//     println!("{}", s2);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }
// Error

// fn main() {
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;
//     let s3 = &s1;
    
//     println!("{}", s1);
//     println!("{}", s2);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }

// No errors
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     print!("User 1 username: {:?}", user1.username);
// }

//Implementing structs

// struct Rect {
//     width: u32,
//     height: u32,
//  }
 
//  impl Rect {
//      fn area(&self) -> u32 {
//           self.width * self.height
//      }
//  }
 
//  fn main() {
//      let rect = Rect {
//          width: 30,
//          height: 50,
//      };
//      print!("The area of the rectangle is {}", rect.area());
//  }

// Define an enum called Shape
// Pattern Matching

// enum Shape {
//     Circle(f64),  // Variant with associated data (radius)
//     Square(f64),  // Variant with associated data (side length)
//     Rectangle(f64, f64),  // Variant with associated data (width, height)
// }

// // Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Square(side_length) => side_length * side_length,
//         Shape::Rectangle(width, height) => width * height,
//     }
// }

// fn main() {
//     // Create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);


//     // Calculate and print the areas
//     println!("Area of circle: {}", calculate_area(circle));
//     println!("Area of square: {}", calculate_area(square));
//     println!("Area of rectangle: {}", calculate_area(rectangle));
    
// }

//Option enum

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("raman");
//     let res = find_first_a(my_string);
//     match res {
//         Some(index ) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string."),
//     }
// }

// use std::fs;
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
// 	let greeting_file_result = fs::read_to_string("hello.txt");
// }

// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("hello.txt");

//     match greeting_file_result {
//         Ok(file_content) => {
//             println!("File read successfully: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("Failed to read file: {:?}", error);
//         }
//     }
// }

// use core::fmt;
// use std::{fmt::{Debug, Formatter}, fs};

// pub struct FileReadError{}

// fn main() {
//     let contents = read_file("hello.txt".to_string());
//     match contents {
//         Ok(file_content) => {
//             println!("File content: {}", file_content);
//         },
//         Err(error) => {
//              println!("Error reading file");
//         }
//     }
// }

// fn read_file(file_path: String) -> Result<String, FileReadError> {
//     let greeting_file_result = fs::read_to_string("hello.txt");
//     match greeting_file_result {
//         Ok(file_content) => {
//             Ok(file_content)
//         },
//         Err(error) => {
//             let err = FileReadError {};
//             Err(err)
//         }
//     }
// }
//Generate a random number

// use rand::{Rng, thread_rng};

// fn main() {
//     let mut rng = thread_rng();
//     let n: u32 = rng.gen();
//     println!("Random number: {}", n);
// }

//Store time in a DB/as a variable

use chrono::{Local, Utc};

fn main() {
    // Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);
}

