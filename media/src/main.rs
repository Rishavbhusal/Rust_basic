
mod content;
use content::catalog::Catalog;
use content::media::Media;
mod option_enum;




                  // understanding the build in enums with this user-defined enums
                  // Those 'a is a lifetimes 
// enum HaveaIndex <'a>{
//   ContainsIndex(&'a Media),
//     NoIndex,
// }
fn  print_media(media:Media){
println!("{:#?}",media)
}


fn main() {
let audiobook = Media::Audiobook { title:String::from("Audio") };
let movie = Media::Movie { title: String::from("Movie"), director: String::from("Rishav") };
let book = Media::Book { title: String::from("Good books"), author:String::from("Good author")};
let podcast = Media::podcast(10);
let placeholdee = Media::placeholder;
let mut catalog =Catalog::new();
// println!("{:#?}",audiobook.description());
println!("{:#?}",book.description());
// println!("{:#?}",movie.description());
catalog.add(audiobook);
catalog.add(book);
catalog.add(movie);
catalog.add(podcast);
catalog.add(placeholdee);


// println!("{:#?}",catalog);

// print_media(audiobook);
// print_media(movie);
// print_media(book);





                            // this is realted to build in enums
// There is a  build in enum called option which looks like:
// enum Option {
//     Some(value),
//     None
// }





// match catalog.items.get(0){
//   Option::Some(value)=>{
//     println!(" Items: {:#?}",value)
//   }
//   Option::None=>{
//     println!("Nothing in this index number")
//   }
// }
// option_enum::function();





match catalog.get_by_index(3) {
Some(value)=>{
  println!("Items : {:#?}",value);
}
 None=>{
println!("there is no index exist");
  }
}
option_enum::function();
    
}
