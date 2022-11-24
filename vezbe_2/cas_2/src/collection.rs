pub mod vector;
mod hashMap;
use std::{io, hash};
const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Unesena vrednost ne moze da se pretvori u broj.";

pub fn collection_menu(){
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - kolekcije");
        println!("1 - Vektor");
        println!("2 - Hash Map");
        println!("0 - Glavni meni");
        println!("__________________________________________________");


        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => vector::vector(),
            2 => hashMap::hash_map(),
            _other => break
        }
    }
}