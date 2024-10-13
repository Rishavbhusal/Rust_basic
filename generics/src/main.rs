use std::primitive;

use num_traits::{ ToPrimitive};

// It is for both argument of float of same type like both are f32 or both are f64


// fn solve <T :Float>(a:T , b:T)->f64{
// let a_64 = a.to_f64().unwrap();
// let b_64 = b.to_f64().unwrap();
// (a_64.powi(2) + b_64.powi(2)).sqrt()
// }


// Below code is used to solve if both argument are of float but they can be both f32 and f64 at a same time



// fn solve<T:Float,U:Float>(a:T,b:U)->f64{
//  let a_64 = a.to_f64().unwrap();
//  let b_64 = b.to_f64().unwrap();
// (a_64.powi(2) + b_64.powi(2)).sqrt()
//  }



// Below function support any types of number either float ,integer , u8 etc 


fn solve<T:ToPrimitive, U:ToPrimitive>(a:T,b:U)->f64{
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();
   ( a_64.powi(2) + b_64.powi(2)).sqrt()
}


fn main(){
    let a:f32 = 22.42;
    let b:u32 = 12;
    println!("{:#?}",solve(a, b))
}