fn main(){
    let x = Some(3u8);
    if let Some(3) = x{
        println!("three");
    }else {
        println!("other");
    }
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     _ => println!("other"),
    // }
}

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }


// fn plus_one(x:Option<i32>) -> Option<i32>{
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// fn main(){
//     let five = Some(5);
//     let y = plus_one(five);
//     let z: Option<i32> = None;
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }


// enum state{
//     A(String),
//     B,
// }
// enum Coin{
//     ShenJie,
//     WangZhanPing,
//     LiChangChun,
//     ChenHao,
//     fun(state),
// }
// fn value_in_cents(coin:Coin)->u8{
//     match coin {
//         Coin::ShenJie => {
//             println!("Luckly! ShenJie");
//             1
//         }
//         Coin::WangZhanPing => 5,
//         Coin::LiChangChun => 2,
//         Coin::ChenHao => 12,
//         Coin::fun(state) => {
//             println!("this state of this enum is {}",state);
//             3
//         }
//     }
// }
// fn main(){
//     value_in_cents(Coin::ShenJie);
//     value_in_cents(Coin::fun(state::A(String::from("hello"))));
// }

// enum Option<T> {
//     None1,
//     Some(T),
// }
// fn main(){
//     // let x1 = Some(String::from("hello"));
//     // let y1:Option<String> = None1;
//     // println!("{}")

//     let x1 = Some(5);
//     let x2 = 5;
//     let x3 = x1 + x2;
// }


// enum Message {
//     Write(String),
//     Quit,
//     Move{x:u32,y:u32},
//     ChangeColor(i32,i32,i32),
// }
// impl Message {
//     fn call(&self){
//         println!("hello");
//     }
// }
// fn main(){
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     heigh: u32,
// }
// impl Rectangle {
//     fn rect() -> Self {
//         Self {
//             width: 30,
//             heigh: 20,
//         }
//     }
//     fn area(&self) -> u32 {
//         self.width * self.heigh
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         heigh: 20,
//     };
//     let rect2 = Rectangle::rect();
//     println!("area = {}, area2 = {}", rect1.area(), rect2.area());
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }
// fn main() {
//     let user1 = build_user(String::from("2609482146@qq.com"), String::from("jienow"));
//     println!("{}",user1.username);
//     let user2 = User{
//         username:String::from("jienow"),
//         ..user1
//     };
//     println!("{}",user2.email);
// }

// fn main() {
//     let user1 = User {
//         email:String::from("2609482146@qq.com"),
//         username:String::from("someusername123"),
//         active:true,
//         sign_in_count:1,
//     };
//     println!("{}",user1.email);
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..3];
//     assert_eq!(slice, &[2, 3]);
// }

// fn first_word(s:&str) -> &str{
//     for (i,&item) in s.as_bytes().iter().enumerate() {
//         if item == b' '{
//             return &s[0..i];
//         };
//     }
//     &s[..]
// }
// fn main(){
//     let str = "hello world";
//     println!("{}\n{}",first_word(&str),first_word(&str[6..]));
// }

// fn first_word(s:&String) -> &str{
//     for (i,&item) in s.as_bytes().iter().enumerate() {
//         if item == b' '{
//             return &s[0..i];
//         };
//     }
//     &s[..]
// }
// fn main(){
//     let str = String::from("hello world");
//     println!("{}",first_word(&str));
// }

// fn first_space(str:&String) -> usize{
//     for (i,&item) in str.as_bytes().iter().enumerate(){
//         if item == b' ' {
//             return i;
//         };
//     }
//     str.len()
// }
// //字符串切片
// fn main(){
//     let str = String::from("hello world");
//     println!("firs word is {}",&str[0..first_space(&str)]);
//     let number = first_space(&str) + 1;
//     println!("second word is {}",&str[number..]);
// }

// fn first_space(str:&String) -> usize{
//     for (i,&item) in str.as_bytes().iter().enumerate(){
//         if item == b' ' {
//             return i;
//         };
//     }
//     str.len()
// }
// fn main(){
//     let str = String::from("hello world");
//     println!("the first space of {str} is {}",first_space(&str));
// }

// fn main(){
//     let mut str = String::from("hello");
//     let s1 = &str;
//     let s2 = &str;

//     println!("s1 :{s1},s2 : {s2}");
//     let s3 = &mut str;
//     println!("s3 :{s3}");
// }

// fn str_add_str(str:&mut String){
//     str.push_str(", world");
// }
// fn main(){
//     let mut str = String::from("hello");
//     str_add_str(&mut str);
//     println!("{str}");
// }

// fn main(){
//     let str = String::from("hello");
//     println!("{}",cal_string_len(&str));

//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();
//     // println!("{}",s1);
//     // println!("{}",s2);

//     // // 如果变量的值只存在在堆上或者栈上，那么深拷贝和浅拷贝效果相同，就可以直接使用
//     // let number1 = 10;
//     // let number2 = number1;
//     // println!("number1 is {number1}");
//     // println!("number2 is {number2}");
// }
// fn cal_string_len(str:&String)->usize{
//     str.len()
// }

// fn main(){
//     let mut s = String::from("hello");
//     s.push_str(", world");
//     println!("{}",s);
// }

//斐波那契
// fn fib(i: u32) -> u32{
//     if i == 1 || i == 2 {
//         return 1;
//     };
//     return 2;
//     // } else {
//     //     return fib(i - 1) + fib(i - 2);
//     // }
// }
// fn main(){
//     let number = fib(10);
//     println!("number is {number}");
//     // let mut i = 1;
//     // let mut j = 1;
//     // for index in (1..10).by_ref(){
//     //     print!("{i} ");
//     //     let number = i;
//     //     i = j;
//     //     j += number;
//     // }
// }

// fn main(){
//     for element in (1..3).by_ref() {
//         println!("element is {element}");
//     }
// }

// fn main(){
//     let arr = [1,2,3,4,5];
//     for i in arr{
//         println!("i is {i}");
//     }

//     // let mut count = 3;
//     // while count != 0 {
//     //     println!("count is {count}");
//     //     count -= 1;
//     // }
// }

// fn main(){
//     let mut number = 0;
//     'countint_up : loop{
//         println!("number is {number}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             } else if number == 2{
//                 break 'countint_up;
//             }
//             remaining -= 1;
//         }
//         number += 1;
//     }
//     println!("number is {number}");
// }

// fn main() {
//     let mut index = 0;
//     let number = loop {
//         index += 1;
//         if index == 10 {
//             break index * 10;
//         }
//     };
//     println!("number is {number}");
// }

// fn main(){
//     let condition = true;
//     let number = if condition {4} else {2};

//     if condition {
//         println!("condition is true");
//     } else {
//         println!("condition is false");
//     }

//     println!("number is {number}");
// }

// fn five() -> u32{
//     5
// }
// fn main(){
//     let x = five();
//     println!("x = {x}");
// }

// fn main(){
//     let y = {
//         let x = 1;
//         x + 1
//     };
//     println!("y = {y}");
// }

// fn add(i:u32,j:u32) -> u32{
//     return i +  j;
// }
// fn main(){
//     let res = add(1,2);
//     println!("{res} 你好");
//     return;
// }

// use std::io;
// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     println!("你好");
//     let mut index = String::new();

//     io::stdin().read_line(&mut index).expect("出错");

//     let index: usize = index.trim().parse().expect("出错");

//     let number= arr[index];

//     println!("arr 的{index} 是{number}");
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let a = arr[0];
//     let arr1: [i32; 5] = [1, 2, 3, 4, 5];
//     let b = arr1[0];
//     let arr2 = [1;5];
//     let c = arr2[0];
//     println!("{a}  {b}  {c} {c}");
// }

// fn main(){
//     let tup : (i32,u32) = (32,2);
//     let (x,y) = tup;
//     let z = tup.0;
//     let h = tup.1;
//     println!("tup is :{x},{y}");
//     println!("tup1 is {z},{h}");
// }

// fn main(){
//     let x = 5;
//     let x = x + 1;
//     println!("x : {x}");

//     {
//         let x = x * 2;
//         println!("x : {x}");
//     }
//     println!("x : {x}");

//     let space = "shell";
//     let space = space.len();
//     println!("space : {space}");
// }

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("让我们来玩猜数游戏!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("请输入数字");
//         let mut number = String::new();

//         io::stdin().read_line(&mut number).expect("读入数据出错");

//         let number: u32 = match number.trim().parse(){
//             Ok(number) => number,
//             Err(_) => {
//                 println!("你输入的数据不是数字");
//                 continue
//             },
//         };

//         match number.cmp(&secret_number) {
//             Ordering::Less => println!("太小了"),
//             Ordering::Greater => println!("太大了"),
//             Ordering::Equal => {
//                 println!("你赢了");
//                 break;
//             }
//         }
//     }
// }

// use std::io;

// fn main(){
//     println!("璁╂垜浠潵鐜╃寽鏁版父鎴�!");
//     let mut guess_number = String::new();
//     io::stdin()
//     .read_line(& mut guess_number)
//     .expect("璇诲叆鏁版嵁鍑洪敊鍟�");
//     println!("浣犵寽鐨勬暟瀛楁槸锛歿guess_number}");
// }
