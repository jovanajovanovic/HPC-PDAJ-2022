use std::collections::HashMap; // da biste koristili mape obavezno morate da uvezete hash mapu iz kolekcije
use std::io;
pub fn hash_map (){
    // mape skladiste elemente u vidu para kljuc, vrednost
    //kreiranje nove mape
    //pr. korisnik unosi koliko elemenata u mapi zeli da ima
    // skladisticemo recenice kao vrednost, a kljuc ce da bude jedna rec

    let mut hash_map = HashMap::new(); //prazna mapa
    // <kljuc, vrednost> = <"hello", "Hello world">

    println!("-------------------------------------");
    println!("Hash Map");
    println!("Unesite velicinu mape: ");

    let mut option = String::new();
     // kod za unos podataka 
     io::stdin()
     .read_line(&mut option) // funkcija koja cita jednu liniju i onda procitanu liniju ubaci u promenljivu option
                             // option - prosledjena kao mut referenca da bi read_line funkcija mogla da menja vrednost promenljive
     .expect("Greska prilikom citanja podataka"); // obrada greske, ako se ona desi u toku citanja podataka koje je korisnik uneo

     //podaci koje korisnik unese su string, a posto su opcije brojevi onda je u sledecoj liniji odradjena konverzija String->int
     let opt:i32 = option.trim().parse().expect("Molimo vas unesite broj.");

     // korisnik je uneo koliki niz zeli, sad trazimo da pojedinacno unese svaki broj
    println!("---------------------------------------");
    println!("Popunjavanje hash mape");
     for x in 0..opt {
        insert_elem_in_map(&mut hash_map);
     }

     println!("---------------------------------------");
     println!("Elementi mape");
     println!("{:?}", hash_map);

    println!("---------------------------------------");
    println!("Pristup elementu mape");
    get_element(&hash_map);


}

fn insert_elem_in_map(map: &mut HashMap<String, String>){
    println!("Unesite kljuc elementa: ");
    let mut key = String::new();

    io::stdin()
    .read_line(&mut key)
    .expect("Greska prilikom citanja podataka"); 

    println!("Unesite recenicu: ");
    let mut line = String::new();

    io::stdin()
    .read_line(&mut line)
    .expect("Greska prilikom citanja podataka"); 

    //postoji 3 nacina za unos elementa u mapu
    // 1. preklapanje postojece vrednosti ako kljuc vec postoji u mapi, upotrebom insert funkcije
    // map.insert(key, line);
    let key = key.trim().to_owned();
    let line:String = line.trim().to_owned();
    //to_owned() - generalizacija clone metode na pozajmljene podatke
    // generalizuje clone tako da konstruise podatke u vlasnistvu iz bilo koje pozajmice datog tipa
    // trim metoda prebaci String u &str i onda sa to_owned vratimo &str u String

    
    // 2. izmena vrednosti elementa na osnovu postojece vrednosti 
    // mozemo napraviti da se spoje dve recenice ako su unesene sa istim kljucem 
    // let elem = map.entry(key).or_insert(String::new());
    // elem.push_str(&line[..]);
    
    
    // 3. unos novog elementa ukoliko ne postoji element sa prosledjenim kljucem 
    // ovaj princip cemo ovde koristiti
    map.entry(key).or_insert(line);
    
}

fn get_element(map: &HashMap<String, String>){
    println!("Unesite kljuc elementa koji zelite da prikazete: ");
    let mut key = String::new();

    io::stdin()
    .read_line(&mut key)
    .expect("Greska prilikom citanja podataka"); 

    let sentence = map.get(&(key.trim().to_owned()));

    match sentence {
        Some(sentence) => println!("Pod kljuce {} se nalazi recenica: {}", key.trim(), sentence),
        None => println!("Ne postoji element pod kljucem {}", key.trim())
    }

}