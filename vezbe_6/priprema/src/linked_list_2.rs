// detaljnu implementaciju mozete pronaci na sajtu https://dev.to/felixfaisal/implementing-linked-list-in-rust-3and
#[derive(Clone)]
pub enum address {
    address(Box<Node>),
    Nil
}

#[derive(Clone)]
pub struct Node {
    pub value: u32, 
    pub next: address, // sledeci element moze da bude ili nista ili novi cvor
}

//struktura je mogla da se napravi i na sledeci nacin
// pub struct Node{
//     value: u32,
//     next: Option<Box<Node>>, //Option moze da bude None ili Box<Node> 
// }

impl Node {
    pub fn new (value: u32, next: address) -> Self{
        Self{
            value, 
            next,
        }
    }
    pub fn append(&mut self, elem: u32){
        match self.next {
            address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            address::Nil => {
                let node = Node {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }

    //brise odredjeni element iz liste
    fn delete(&mut self, elem: u32){
        match self.next {
            address::address(ref mut next_address) => {
                if(next_address.value == elem){
                    println!("Deleting value: {}", next_address.value );
                    self.next = next_address.next.clone();
                }else {
                    next_address.delete(elem);
                }
            }
            address::Nil => {
                if self.value == elem {
                    self.value = 0;
                }else {
                    println!("Element {} does not exist in the linked list.", elem);
                }
            }
        }
    }

    //broji elemente liste
    fn count(&self) -> u32 {
        match self.next {
            address::address(ref new_address) => 1 + new_address.count(),
            address::Nil => 0   
        }
    }

    //ispis liste
    pub fn print_list(&self){
        if self.value == 0 {
            println!("List is empty");
        }else {
            println!("{}", self.value);
            match self.next {
                address::address(ref next) => next.print_list(),
                address::Nil => {}
            }
        }
    }
}