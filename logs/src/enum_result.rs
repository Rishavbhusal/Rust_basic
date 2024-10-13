
use std::io::Error;
pub fn function(){

 match division(5.00, 0.0){
    Ok(result)=>{
        println!("{}",result)
    }
    Err(error)=>{
        println!("{}",error)
    }
};



 pub fn division(a:f64, b:f64)->Result<f64,Error>{
    if b == 0.00{
        Err(Error::other("Can't divide the number by 0"))
    }else 
    {
        Ok(a/b) 
    }
    
    
}


match email_validate(String::from("johndoe@gmail.com")){
    Ok(..)=>{
println!("Email is valid!!")
    }
    Err(reason_for_error)=>{
        println!("{}",reason_for_error)
    }
}



 pub fn email_validate(email:String)->Result<(),Error>{
    if email.contains("@"){
        Ok(())
    }else{
        Err(Error::other("Email isnt valid"))
    }
} 
}