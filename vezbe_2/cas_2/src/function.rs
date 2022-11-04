use std::io;

const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Molimo vas unesite broj.";

pub fn fun_menu(){
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - funkcije");
        println!("1 - Funkcija bez povratne vrednosti");
        println!("2 - Funkcija sa povratnom vrednoscu");
        println!("0 - Glavni meni");
        println!("__________________________________________________");


        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => fun_without_return_value(),
            2 => fun_with_return_value(),
            _other => break
        }
    }
}

fn fun_without_return_value() {
    println!("----------------------------------------------------");
    println!("Funkcija bez povratne vrednosti");
    println!("Funkcija ispisuje vrednosti promenljivih");
    //u okviru funkcije mozete da imate izraze i iskaze
    // iskazi - naredbe, kao sto su: 
    let a = 5; 
    println!("a = {a}");
    // izrazi - izracunava se do rezultujuce vrednosti i moze da bude sastavni deo iskaza
    let b = a + 3; // a + 3 => izraz
    println!("b = {b}");
}

fn fun_with_return_value() {
    println!("----------------------------------------------------");
    println!("Funkcije sa povratnom vrednoscu");
    println!("a + b = {}", sum(23, 56));
    println!("a * b = {}", mul(23, 56));
}

// parametri funkcije se navode u zagradama posle naziva funkcije, treba da se definise ime i tip parametra
//ako funkcija ima povratnu vrednost onda tip povratne vrednost treba da bude napisan odmah posle liste parametara
fn sum(a: i32, b:i32) -> i32{
    println!("{}",a);
    println!("{b}");
    a + b //ako nema kljucne reci return onda ce iz funkcije biti vracen poslednji izraz koji je napisan bez ; na kraju
}

fn mul(a: i32, b: i32) -> i32{
    println!("{a}");
    println!("{b}");
    return a*b; //mozete i na ovaj nacin da vratite vrednost, ali onda na kraju mora da stoji ;
}