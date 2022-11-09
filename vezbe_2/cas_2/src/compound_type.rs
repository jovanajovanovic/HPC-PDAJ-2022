use std::io;
mod array;
mod tuple;

const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Unesena vrednost ne moze da se pretvori u broj.";

pub fn compound_type_menu(){
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - slozeni tipovi podataka");
        println!("1 - Niz");
        println!("2 - Tuple");
        println!("0 - Glavni meni");
        println!("__________________________________________________");


        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => array::array_menu(),
            2 => tuple::tuple_menu(),
            _other => break
        }
    }
}