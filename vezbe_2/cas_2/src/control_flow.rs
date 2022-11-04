//posto cemo koristiti funkciju za random brojeve onda prvo moramo da importujemo biblioteku rand:
// 1. u sekciju dependencies u Cargo.toml fajl dodati ime i verziju biblioteke
// 2. importovati biblioteku tamo gde zelite da je koristite upotrebom use sintakse
use rand::Rng; // Rng - interfejs u kojem je importovana metoda za generisanje random brojeva
use std::io;


const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Molimo vas unesite broj.";

pub fn control_flow_menu(){
    // ova konstrukcija je vec koriscena na vise mesta
    // loop - petlja koja se izvrsava beskonacno puta, moze da bude prekinuta sa CTRL+C ili upotrebom break
    // ovde je prekinuta sa break, koji se poziva kada se ne unese jedna od ponudjenih opcija menija
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - kontrole toka i petlje");
        println!("1 - if");
        println!("2 - while");
        println!("3 - for");
        println!("0 - Glavni meni");
        println!("__________________________________________________");
        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => if_stmt(),
            2 => while_stmt(),
            3 => for_stmt(),
            _other => break
        }
    }
}

fn if_stmt(){
    println!("----------------------------------------------------");
    println!("IF");
    println!("Provera da li je random broj deljiv sa 2 ili sa 5");
    // primer: provera da li je random broj deljiv sa 2, sa 5 ili nije deljiv ni sa 2 ni sa 5 -> u projekat je ubacena biblioteka rand
    // generisemo random broj
    let rand_num = rand::thread_rng().gen_range(1..=30); // gen_range metoda prima parametar koji predstavlja opseg u kojem ce biti generisani random brojevi 
                                                          // start..=end => 1..=30 (u ovom ovde primeru ce biti generisani brojevi od 1 do 30)

    if rand_num % 2 == 0 {
        println!("Broj {rand_num} je deljiv sa 2.");
    }else if rand_num % 5 == 0{
        println!("Broj {rand_num} je deljiv sa 5.");
    }else {
        println!("Broj {rand_num} nije deljiv ni sa 2 a ni sa 5.");
    }
}

fn while_stmt(){
    // primer: sabiranje prvih 30 brojeva 
    println!("----------------------------------------------------");
    println!("WHILE");
    println!("Sabiranje prvih 30 brojeva");
    let mut sum = 0;
    let mut counter = 1;
    const LIMIT: i32 = 30;
    while counter < LIMIT {
        sum += counter;
        counter += 1;
    }

    println!("Zbir prvih 30 brojeva je: {sum}");
}

fn for_stmt(){
    println!("----------------------------------------------------");
    println!("FOR");
    println!("Prikaz elemenata niza");
    //kao i u bilo kom drugom jeziku for se koristi za prolazak kroz kolekciju ili opseg nekih vrednosti
    // primer 1: ispisati sve elemente niza
    let a = ['a', 'b', 'c', 'd', 'e'];
    for elem in a {
        println!("{elem}");
    }
    println!("----------------------------------------------------");
    println!("Parni brojevi prve stotine");
    // primer 2: ispisati sve parne brojeve prve stotine 
    for num in 1..100 {
        if num % 2 == 0 {
            println!("{num}");
        }
    }
    println!("----------------------------------------------------");
    println!("Brojevi prve desetice u obrnutom redosledu");

    //primer 3: ispisati brojeve prve desetice u obrnutom redosledu
    for n in (1..10).rev() { // end=10, zato sto se ne ukljucuje taj broj u opseg
        println!("{n}");
    }
}
