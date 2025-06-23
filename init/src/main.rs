// // use chrono::prelude::*;
// use chrono:: {Local, Utc};

// fn main() {
//     let utc = Utc::now();
//     let local_time =  Local::now();

//     print!("{}", utc);
//     print!("{}", local_time);
// }

// use dotenv::dotenv;
// use std::env;

// fn main() {
//     dotenv().ok();
//     let redis_url = env::var("REDIS_ADDRESS");

//     let redis_url_result = redis_url.unwrap();

//     println!("{}", redis_url_result)

//     // match var {
//     //     Ok(str) => println!("{}", str),
//     //     Err(_e) => println!("Error while reading variable"),
//     // }
// }

// #[derive(Debug)]
// struct User {
//     username: String,
// }

// fn main() {
//     let u = User {
//         username: String::from("Sarfraz Qadri"),
//     };

//     println!("{}", u.username);
//     println!("{:?}", u);
// }

// struct User {
//     username: String,
// }

// fn main() {
//     let u1 = User {
//         username: String::from("Muskan"),
//     };
//     let u2 = User {
//         username: String::from("Muskan"),
//     };
//     display_element(1, 3);
//     display_element(String::from("Sarfraz"), String::from("Alam"));
// }

// fn display_element<T: std::fmt::Display>(a: T, b: T) {
//     print!("{}", a);
//     print!("{}", b);
// }

// struct User{
//     username:String
// }

// fn main(){
//     let u1 = User{
//         username:String::from("Sarfraz Khan")
//     };
//     print_variable(1);
//     print_variable(1.2);
//     print_variable(String::from("Sarfraz"));
// }

// fn print_variable<T:std::fmt::Display>(a:T){
//     println!("{}", a)
// }

///////////////////////////////////////////////////////////Generics
///
// struct Rect<T> {
//     height: T,
//     width: T,
// }

// impl<T: std::ops::Mul<Output = T>+Copy> Rect<T> {
//     fn area(&self) -> T {
//         return self.height * self.width;
//     }
// }

// fn main() {
//     let r = Rect {
//         height: 12,
//         width: 10,
//     };

//     let r2 = Rect {
//         height: 10.0,
//         width: 12.0,
//     };

//     print!("{}", r.area());
//     print!("{}", r2.area());
// }

// struct Rect{
//     height:u32,
//     width:u32
// }

// impl Rect {
//     fn area(&self)->u32{
//         return self.height*self.width;
//     }
// }

// fn main(){
//     let r = Rect{
//         height:12,
//         width:10
//     };

//     print!("{}", r.area());
// }

// struct Rect<T> {
//     height: T,
//     width: T,
// }

// impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
//     fn area(&self) -> T {
//         return self.height * self.width;
//     }
// }

// fn main() {
//     let u = Rect {
//         height: 10,
//         width: 12,
//     };

//     let u2 = Rect {
//         height: 10.9,
//         width: 3.5,
//     };

//     print!("{}", u.area());
//     print!("{}", u2.area());
// }

// use std::f32::consts::PI;

// ///////////////////////////////////////////////////////  trait
// ///

// trait Shape {
//     fn area(&self)->f32;
// }
// struct Rect {
//     height:f32,
//     width:f32
// }

// impl Shape for Rect {
//     fn area(&self)->f32{
//         return self.height*self.width;
//     }
// }
// struct Circle {
//     radius:f32
// }

// impl Shape for Circle {
//     fn area(&self)->f32{
//         return self.radius*self.radius *3.14;
//     }
// }

// fn print_area_of_shape<T:Shape>(s:T){
//     print!("{}",s.area());
// }

// fn main(){
//     let r = Rect{
//         height:10.0,
//         width:10.0
//     };

//     let c = Circle{
//         radius:10.0
//     };

//     print_area_of_shape(r);
//     print_area_of_shape(c);
// }

// trait Shape {
//     fn area(&self) -> f32;
// }

// struct Rect {
//     height: f32,
//     width: f32,
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.height * self.width;
//     }
// }

// struct Circle {
//     radius: f32,
// }

// impl Shape for Circle {
//     fn area(&self) -> f32 {
//         return PI * self.radius * self.radius;
//     }
// }

// fn print_area_of_shape<T: Shape>(s: T) {
//     println!("{}", s.area());
// }

// fn main() {
//     let r = Rect {
//         height: 10.0,
//         width: 3.9,
//     };

//     let c = Circle { radius: 7.0 };

//     println!("{}", r.area());
//     println!("{}", c.area());

//     print_area_of_shape(r);
//     print_area_of_shape(c);
// }

////////////////////////////////////////////////////////// macros

// fn main(){
//     // println!("Hello Sarfraz");
//     let v = vec![1,2,3];
//     print!("{:?}", v);
// }

// macro_rules! say_hello {
//     () => {
//         println!("Hello, world!");
//     };
// }

// fn main(){
//     say_hello!();
//     println!("Hello, world!")
// }

// use std::fmt::{write, Debug, Display};

// #[derive(Debug)]
// struct User{
//     username:String,
//     password:String,
//     age:u32,
// }

// impl Display for User {
//     fn fmt(&self, f:&mut std::fmt::Formatter)->std::fmt::Result{
//         write!(f, "This is the user struct with age {}", self.age)
//     }
//  }

// fn main(){
//     let u = User{
//         username:String::from("Sarfraz"),
//         password:String::from("123231"),
//         age:20
//     };

//     print!("{:?}", u);   //debug
//     print!("{}", u);   //display
// }

// #[derive(Debug, Clone, Copy)]
// struct User {
//     is_male:bool,
//     age:u32
// }

// fn main(){
//     let u1 = User{
//         is_male:true,
//         age:20
//     };

//     let u2 = u1;

//     print!("{:?} {:?}", u1, u2);
// }

// /////////////////////////////////////////////////////////////////////////////////////////////// Serde ///////////////////////////////////////
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug,Clone)]
// struct User {
//     username: String,
//     password: String,
// }

// fn main() {
//     // // Serialization

//     // let u = User {
//     //     username: String::from("Sarfraz"),
//     //     password: String::from("123321"),
//     // };

//     // let serialized_string = serde_json::to_string(&u);
//     // let user_string = serialized_string.unwrap();
//     // // println!("{}", user_string);
//       // match serialized_string {
//     //     Ok(str) => println!("{}", str),
//     //     Err(_) => println!("Error while converting to string"),
//     // }

//     // // Deserialization
//     // let s = String::from("{\"username\":\"Sarfraz\", \"password\":\"12321\"}");
//     // let u: Result<User, serde_json::Error> = serde_json::from_str(&s);

//     // let user_struct = u.unwrap();
//     // println!("{:?}", user_struct);

// }

// /////////////////////////////////////////////////////////////////////////////////////////////// Borsh ///////////////////////////////////////
// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
// struct User {
//     username: String,
//     password: String,
// }

// fn main() {
//     let u = User {
//         username: String::from("Sarfraz"),
//         password: String::from("123243"),
//     };

//     let mut v: Vec<u8> = Vec::new();
//     // serialize
//     let ans = u.serialize(&mut v);
//     // deserialize
//     let user = User::try_from_slice(&v).unwrap();

//     print!("{}", user.username);
// }

// /////////////////////////////////////////////////////////////////////////  lifetimes /////////////////////

fn main() {
    let str1 = String::from("Sarfraz");
    let ans;
    {
        let str2 = String::from("Qadri");
        ans = longest_string(&str1, &str2);
    }

    println!("{}", ans);
}

fn longest_string<'a, 'b>(s1: &'a String, s2: &'b String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}
