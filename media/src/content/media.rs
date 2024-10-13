#[derive(Debug)]


pub enum Media {
    Book {title:String,author:String},
    Movie {title:String,director:String},
    Audiobook{title:String},
    podcast (u32),
    placeholder,
    
}
impl Media {
  pub   fn description(&self)->String{


        // Method -1

  // if let Media::Book { title, author } =self{
  //   format!("Book: {} {}",title,author)
    
  // } else if let Media::Movie{title,Director} = self{
  //   format!("Movie: {} {}",title,Director)
  // }
  // else if let Media::Audiobook { title }=self{
  //   format!("Audiobook: {}",title)
  // }
  // else{
  //   String::from("Media description")
  // }

                //   Method -2



  match self {
    Media::Book { title, author }=>{
      format!("Book: {} {}",title,author)
    }
    Media::Movie { title, director }=>{
      format!("Movie : {} {} ", title,director)

    }
    Media::Audiobook { title }=>{
      format!("Audiobook {}",title)
    }
    Media::podcast(ep_num)=>{
      format!("podcast{}",ep_num)
    }
    Media::placeholder=>{
      format!("Placeholder")
    }
  }
    }
}