mod acc;

use acc::function;
use std::string;
#[derive(Debug)]
struct Account {
    id:u32,
    balance: i32,
    holder:String
}

impl Account{
    fn new(id:u32, holder:String)->Self {
Account { id, balance:0, holder }

        
    }

    fn summary(&self)->String{
        format!("{} has a balance {}",self.holder,self.balance)

    }

    fn deposit(&mut self, amount:i32)->i32{
self.balance+= amount;
self.balance
    }
    fn withdraw(&mut self, amount:i32)->i32{
        self.balance -= amount;
        self.balance

    }
}
#[derive(Debug)]

struct Bank{
    accounts :Vec<Account>
}

impl Bank{
    fn new()->Self {
        Bank { accounts:vec![] }
    }
  
}

// reading
fn print_account(account:&Account){
    println!("{:#?}",account);

}

fn print_holder(holder: String){
println!("{}",holder);
}

// writing 

fn change_account(account: &mut Account){
    account.balance = 30;
    // println!("{:#?}",account.holder);
}
fn main() {
let mut bank = Bank::new();
let mut  account = Account::new(1,String::from("Rishav"));
// println!("{:#?}",bank); 
// account = print_holder(account.holder);

// // let account_ref = &account;
// print_account(&account);
//  reading
println!("{:#?}",&account.holder);

//  writing

change_account(&mut account);
print_account(&account);
println!("{:#?}",account);

}
