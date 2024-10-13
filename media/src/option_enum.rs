#[derive(Debug)]


enum Media {
    Book {title:String,author:String},
    Movie {title:String,director:String},
    Audiobook{title:String},
    podcast(u32),
    placeholder,
    
}




#[derive(Debug)]
struct Catalog{
  items:Vec<Media>
}
impl Catalog {
    fn new()->Self{
Catalog{items:vec![]}
    }
fn add(& mut self, media :Media){
  self.items.push(media);
}

fn get_by_index(&self, index:usize)->Option<&Media>{
if self.items.len()>index{
  Some(&self.items[index])
}else{
  None
}

}
}


pub fn function(){
  let audiobook = Media::Audiobook { title:String::from("Audio") };
let movie = Media::Movie { title: String::from("Movie"), director: String::from("Rishav") };
let book = Media::Book { title: String::from("Good books"), author:String::from("Good author")};
let podcast = Media::podcast(10);
let placeholdee = Media::placeholder;
let mut catalog = Catalog::new();

catalog.add(audiobook);
catalog.add(book);
catalog.add(movie);
catalog.add(podcast);
catalog.add(placeholdee);

let item = catalog.get_by_index(2324);
let placeholdered = Media::placeholder;


// unwrap le some vayo vanne value natra noone vaye main panic
// println!("{:#?}",item.unwrap());


// expect le  some vayo vanne value natra main panic with  some msg
// println!("{:#?}",item.expect("There should be a value"))


// unwrap_or le chahi value vaye value natra if none vayo vanne we can show some message instead of main psnic
println!("{:#?}",item.unwrap_or(&placeholdered))

}