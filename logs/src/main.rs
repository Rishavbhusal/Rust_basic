use std::fs;
use std::io::Error;

// mod enum_result;

fn extract_error(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut result = vec![];
    for items in split_text {
        if items.starts_with("ERROR") {
            result.push(items);
        }
    }
    result
}

// Third method needs a return type in main



// only third method need a return type in main

fn main()->Result<(),Error>{
  //  Third method 
let text = fs::read_to_string("logs.txt")?;
let error_logs = extract_error(text.as_str());
fs::write("error.txt", error_logs.join("\n"))?;
Ok(())

// enum_result::function();
}




                               // second mathod using expect
// let text = fs::read_to_string("logs.txt").expect("Failed to read log file!!!");
// let error_logs = extract_error(text.as_str());
// println!("The errors are: {:#?}",error_logs);
// fs::write("error.txt", error_logs.join("\n")).expect("Failed to write errors:");











                                     // first method using match 
// match  fs::read_to_string("logs.txt"){
//     Ok(result)=>{
//      let error_logs = extract_error(result.as_str());
//     //  println!("{:#?}",error);
//         // println!("{:#?}",result.len())

//        match fs::write("error.txt",error_logs.join("\n")){
//         Ok(..)=>{
//           println!("error are written!!!")
//         }
//         Err(error)=>{
//           println!("Error writing logs: {}",error)
//         }
//        }
//     }
//     Err(error)=>{
//       println!("Failed to load file: {}",error)
//     }
// }


    
