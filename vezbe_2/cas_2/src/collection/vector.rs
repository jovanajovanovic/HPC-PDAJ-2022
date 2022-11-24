use std::io;
pub fn vector(){
    // za razliku od niza vektori su dinamicki, 
    //definisanje vektora
    let mut v: Vec<i32> = Vec::new(); // prazan vektor
    // crate::collection::collection_menu(); //apsolutna putanja
    // collection::collection_menu;
    // super::super::menu();
    //pr. korisnik ce da unese elemente vektora
    println!("-------------------------------------");
    println!("Vektor");
    println!("Unesite velicinu vektora: ");

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
    println!("Popunjavanje vektora");
     for x in 0..opt {
        insert_elem_in_vector(&mut v);
     }

     println!("---------------------------------------");
     println!("Elementi vektora");
     println!("{:?}", v);

    println!("---------------------------------------");
    println!("Pristup elementima vektora");
    get_element(&v);
     
    
}

fn insert_elem_in_vector(v : &mut Vec<i32>){
    //funkcija koja unosi novi element u vektor
    println!("Unesite jedan broj: ");
    let mut line = String::new();

    io::stdin()
    .read_line(&mut line)
    .expect("Greska prilikom citanja podataka"); 

    let elem:i32 = line.trim().parse().expect("Molimo vas unesite broj.");

    v.push(elem); //push - funkcija za unos elementa u vektor
} 


fn get_element(v: &Vec<i32>){
    println!("Unesite indeks elementa koji zelite da prikazete: ");
    let mut line = String::new();

    io::stdin()
    .read_line(&mut line)
    .expect("Greska prilikom citanja podataka"); 

    let idx:usize = line.trim().parse().expect("Molimo vas unesite broj.");

    let elem: Option<&i32> = v.get(idx);
    // get metoda vraca element na nekoj poziciji
    // za ovo moze da se koriste i [], ali vodite racuna ovo nije sigurno jer ako pokusate da pristupite elementu na poziciji koja ne postoji vas program ce poceti da panici
    //OPTION - enum, cije vrednosti mogu da budu Some(T) ili None
    //Rust nema null, ovo resava taj problem 

    match elem {
        Some(elem) => println!("Na indeksu {idx} se nalazi element: {}", elem),
        None => println!("Na indeksu {idx} ne postoji element")
    }

}