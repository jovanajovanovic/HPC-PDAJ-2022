use std::io;

const READ_LINE_ERROR: &str = "Greska prilikom citanja podataka";
const PARSE_STRING_TO_INT_ERROR: &str = "Unesena vrednost ne moze da se pretvori u broj.";

pub fn array_menu(){
    loop  {
        let mut option: String = String::new();
        println!("==================================================");
        println!("Meni - nizovi");
        println!("1 - Kreiranje i prikaz elemenata niza");
        println!("2 - Pristup elementu niza");
        println!("0 - Glavni meni");
        println!("__________________________________________________");


        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect(READ_LINE_ERROR); 

        let opt:i32 = option.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        match opt { 
            1 => createArray(),
            2 => getElement(),
            _other => break
        }
    }
}

fn createArray(){
    println!("----------------------------------------------------");
    println!("NIZ");
    println!("Kreiranje niza i prikaz elemenata niza");
    // svi elementi u nizu moraju da budu istog tipa; velicina niza ne moze da se menja 
    let a: [i32; 5] = [10, 20, 30, 40, 50]; //[tip, broj elemenata ]

    println!("a = {:?} ", a);
    println!("Duzina niza a je: {} ", a.len()); 

    let s = ['a', 'b', 'c', 'd', 'e', 'f']; //jos jedan nacin za definisanje niza, na osnovu elemenata niza kompajler ce zakljuciti kog su tipa elementi u nizu
    println!("s = {:?} ", s); 

    //niz sa podrazumevanim elementima 
    let b: [i32; 4] = [-1; 4]; // [podrazumevana vrednost, broj elemenata u nizu]
    println!("b = {:?} ", b);

}

 fn getElement(){
    //Primer: Kreiracemo niz od 4 elementa. Elemente niza ce korisnik sam da unese. Prikazacemo ceo niz, a zatim element na indeksu koji korisnik unese. 

    let mut arr: [i32; 4] = [-1; 4]; 

    for i in 0..4{
        println!("Element na indeksu {i}: ");

        let mut elem = String::new();
        io::stdin()
        .read_line(&mut elem)
        .expect(READ_LINE_ERROR); 

        let element:i32 = elem.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);

        arr[i] = element;
    }

    //prikaz elemenata niza pomocu for petlje i iter funkcije 
    println!("--------------------------------------------------------");
    println!("Niz: ");
    for el in arr.iter(){
        println!("{}", el);
    } 

    println!("--------------------------------------------------------");
    //pristup elementima niza 
    println!("Unesite indeks elementa koji zelite da prikazete: ");
    let mut index = String::new();
    io::stdin()
    .read_line(&mut index)
    .expect(READ_LINE_ERROR); 

    let ind:usize = index.trim().parse().expect(PARSE_STRING_TO_INT_ERROR);
    
    let e = if ind < arr.len() { arr[ind]} else {-1};
    if e == -1 {
        println!("Na indeksu {ind} ne postoji element. Napomena: sigurno ste uneli indeks koji je veci od duzine niza");
    }else {
        println!("Na indeksu {ind} se nalazi element: {}", e);
    }

}