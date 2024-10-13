
fn next_lang <'a>(language:&'a[String],select:&str)->&'a str{
    let mut found = false;
for lang in language{
    if found{ 
        return lang;
    }
    if lang == select{
        found = true;
    }
}
language.last().unwrap()
}


fn longest_language<'a>(lang1:&'a str,lang2 :&'a str)->& 'a str{
if lang1.len()>= lang2.len(){
    return lang1;
}else {
    return lang2;
}
}
  
fn main(){
    let languages = vec![
    String::from("Rust"),
    String::from("Go"),
    String::from("Typescript")
    ];

//    let next_language =next_lang(&languages, "Rust");
//    println!("{:#?}",next_language);

let result = longest_language("Javascript", "Rust");
println!("{:#?}",result)
}