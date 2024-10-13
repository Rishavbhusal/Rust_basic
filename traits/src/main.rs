mod basket;
mod stack;
mod container;
use basket::Basket;

use container::Container;

use stack::Stack;

fn add_string<T:Container<String>>(c:&mut T, s:String){
    c.put(s);
}
fn main(){
let mut new = Basket::new(String::from("Hello"));
let new2= Basket::new(2);
let new3 = Basket::new(true);

let stack1 = Stack::new(vec!["hello", "dean"]);
let stack1 = Stack::new(vec![3,5,6]);

println!("{:#?}",add_string(&mut new, String::from("hello")));
}