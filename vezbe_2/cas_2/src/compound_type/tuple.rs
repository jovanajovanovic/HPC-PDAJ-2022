use std::io;

const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Unesena vrednost ne moze da se pretvori u broj.";

pub fn tuple_menu(){
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - torka");
        println!("1 - Kreiranje i prikaz torke");
        println!("0 - Glavni meni");
        println!("__________________________________________________");


        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => createTuple(),
            _other => break
        }
    }
}

fn createTuple(){
    println!("----------------------------------------------------");
    println!("TORKA");
    println!("Kreiranje torke i prikaz elemenata torke");
    // elementi u torci ne moraju da budu istog tipa, za svaki element pojedinacno se navodi kog je tipa
    let t: (i32, f64, char) = (34, 9.57, 's');

    println!("t = {:?} ", t);

    //destrukturiranje torke, sablon za izvlacenje vrednosti koje se nalaze u torci

    let (i, f, c) = t; 
    println!("Destrukturiranje torke: i - {}, f - {}, c - {}", i, f, c);

    println!("---------------------------------------------------");
    //pristup vrednostima torke na osnovu indeksa
    println!("i32 - {}", t.0);
    println!("f64 - {}", t.1);
    println!("char - {}", t.2);

}