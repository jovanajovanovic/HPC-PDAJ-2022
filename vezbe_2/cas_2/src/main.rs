mod variable;
mod function;
mod control_flow;
mod compound_type;
pub mod collection;
use std::io;
///
/// Projekat u kojem su dati primeri vezani za drugi termin vezbe iz predmeta Paralelne i distribuirane arhitekture i jezici. 
/// Teme:
/// 1. unos podataka putem konzole
/// 2. deklarisanje promenljivih
/// 3. tipovi podataka
/// 4. kolekcije podataka [vektor, hash map] 

/// Funkcija za ispis meni-a, izborom neke stavke menia bice prikazana demonstracija odredjene teme. 

fn menu(){
    loop  {
        let mut option: String = String::new();
        println!("Meni");
        println!("1 - Promenljive");
        println!("2 - Funkcije");
        println!("3 - Kontrole toka");
        println!("4 - Slozeni tipovi podataka");
        println!("5 - Kolekcije podataka");
        println!("0 - Izlaz iz programa");

        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option) // funkcija koja cita jednu liniju i onda procitanu liniju ubaci u promenljivu option
                                // option - prosledjena kao mut referenca da bi read_line funkcija mogla da menja vrednost promenljive
        .expect("Greska prilikom citanja podataka"); // obrada greske, ako se ona desi u toku citanja podataka koje je korisnik uneo

        //podaci koje korisnik unese su string, a posto su opcije brojevi onda je u sledecoj liniji odradjena konverzija String->int
        let opt:i32 = option.trim().parse().expect("Molimo vas unesite broj.");

        // match -> switch u drugim jezicima, sa _other ili _ umesto default statementa
        match opt { 
            1 => variable::var_menu(),
            2 => function::fun_menu(),   
            3 => control_flow::control_flow_menu(),
            4 => compound_type::compound_type_menu(), //relativna putanja
            5 => crate::collection::collection_menu(),//apsolutna putanja
            _other => break
        }
    }
}

fn main() {
    menu();
}
