use std::{f32::consts::E, fmt::format};
                            //   slice of vector
fn print_element(elements:&[String]){
// fn print_element(elements:&Vec<String>){
// for element in elements{
//  println!("{}",element);
// }
elements.iter().map(|e| format!("{} {}",e,e)).for_each(|e| println!("{}",e));
// .for_each(|e| println!("{}",e));     
}


// note: yeti xutti element banaune ho vanne map ma hunxa ra yeti tehi element bhitra modify garne vayye constructor vitra hunxa 




fn shorten_elements(elements:&mut[String]){
elements.iter_mut().for_each(|e| e.truncate(1));
}


fn to_uppercase(elements:&[String])->Vec<String>{
elements.iter()
.map(|e| e.to_uppercase())
// it is ok to write in both way in return and or directly using :: to collect 
.collect::<Vec<String>>()
}

fn move_value(a:Vec<String>,b:&mut Vec<String>){
a.into_iter().for_each(|e| b.push(e));
}

fn explode(elements:&[String])->Vec<Vec<String>>{
  elements.iter().map(|e| e.chars().map(|c| c.to_string()).collect()
).collect()
}


fn find_color(element:&[String],search:&str,fallback:&str)->String{
  element.iter().find(|e| e.contains(search)).map_or(String::from(fallback), |e| e.to_string())
}
fn main(){
    let mut color = vec![String::from("Red"),String::from("yellow"),String::from("green")];
    // let mut colors_iter = color.iter();
    // println!("{:#?}",colors_iter.next());
    // println!("{:#?}",colors_iter.next());
    // println!("{:#?}",colors_iter.next());
    // println!("{:#?}",colors_iter.next());



    // print_element(&color[1..3]);
    // shorten_elements(&mut color);
  // println!("{:?}",to_uppercase(&color));
//   println!("{:#?}",color)


// move_value 
// let mut destination = vec![];
// move_value(color,&mut destination);
// println!(" Destination : {:#?}",destination)


// explode    

// println!("{:#?}",explode(&color));

// find_color 

let find_color = find_color(&color, "Red", "orange");

println!("{:#?}",find_color)
}
