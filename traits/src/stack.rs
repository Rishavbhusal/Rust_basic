use super::container::Container;


pub struct Stack<T> {
    item : Vec<T>,
}
impl <T> Stack<T>{
    pub fn new(item:Vec<T>)->Self{
        Stack{item}
            }
}

impl  <T> Container <T> for Stack<T>{
   
     fn get(&mut self)->Option<T>{
self.item.pop()
    }
     fn put(&mut self, item:T){
        self.item.push(item);
    }
     fn is_empty(&self)->bool{
        self.item.is_empty()
    }
}