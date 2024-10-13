 use rand::{seq::SliceRandom, thread_rng};
// Debug is known as traits in this struct
// Traits are the set of function
// It gives rus comiler some extra instruction
 // This whole statement is defines as attributes for Deck struct
 #[derive(Debug)]

 struct Deck{
    cards : Vec<String>,
 }

//  Inherent Implementation
impl Deck {
    // ->self shows this function return Deck itself
    // Associated Function
    // If this function containes of reference of self (&self) in () then it is called as methods 
    fn new()->Self {
        
    // List of suits - HEart,Spades,
    // List of value - ace, 
    // Its value didnt varies so we can use arrays instead of vector
    let suits = ["Hearts ♥","Diamonds ♦","Clubs ♣","Spades ♠"];
    let values = ["Ace","Two","Three","Four","Five","Six","Seven","Eight","Nine","Ten","Jack (J)","Queen (Q)","King (K)"];

 let mut cards = vec![];

    for suit in suits{
        for value in values{
            let card =format!("{} of {}",value,suit);
            cards.push(card);
        }
    }
    // variable is known as binding in rust
    let   deck = Deck{cards:cards};
    return deck;

    // we can also return deck in implicit by simply doing this

    // at line number 35 we can do directly Deck{cards} and it will automattically retun deck
        
    }

fn suffle(&mut self){
    let mut rng = thread_rng();
    self.cards.shuffle(&mut rng)
}   
fn deal(&mut self,num_cards:usize)->Vec<String>{
    self.cards.split_off(self.cards.len()-num_cards)
                        //   8-4 == 4
                    //   9-4 == 5 after 5 there are 4 cards to be equvalent to 9

}
}
fn main() {


let mut deck = Deck::new();
deck.suffle();
let cards = deck.deal(3);


println!("Heres the deal :{:#?}",cards);
    // :? is a string debug formatter
    // The # does work as \n for formatting
    println!("Heres the deck :{:#?}",deck);
    // !is known as macros in rust
    println!("Hello world")

}


// IN rust package is called crate
