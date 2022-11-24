// implementacija jednostruko spregnute liste 
// implementacija je uradjena po ugledu na implementaciju sa sajta: https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676, sa malim izmenama

// elem => data, next 

pub enum Link {
    None,
    Tail {item: i32},
    Link {item: i32, next: Box<Link>}
}

impl Link {
    pub fn push(&mut self, x: i32){
        // x - podataka koji se upisuje
        match self{
            Self::None => self.to_tail(x),
            Self::Tail{ .. } => self.to_link(x),
            Self::Link {next, ..}=> next.push(x) //pomeramo se sve dok ne pronadjemo tail cvor
        }
    }

    // funkcija pretvara neki cvor u tail
    fn to_tail (&mut self, it: i32){
        *self = match self{
            Self::None => Self::Tail {item: it},
            Self::Link {item:_, next:_} => Self::Tail {item: it},
            _ => panic!("Could not convert to tail")
            
            
        }
    } 


    //funkcija pretvara tail cvor u link
    fn to_link (&mut self, it: i32){
        *self = match self{
            Self::Tail{item} => {
                Self::Link {item: *item, next: Box::new(Self::Tail{item: it})}},
            
            _ => panic!("Could not convert to link")
            
            
        };
    } 

    pub fn pop(&mut self) -> Option<i32> {
        match self{
            Self::None => None, 
            Self::Tail {item} => {
                let item = *item;
                self.to_none();
                Some(item)
            },
            Self::Link {item, next} => {
                let mut n = Box::new(Self::None);
                let item = *item; 
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                Some(item)
            },
        }
    }

    fn to_none(&mut self){
        *self = std::mem::replace(self, Link::None);
    }

    fn to_next(&mut self, nxt: Link){
        *self = nxt;
    }
}
