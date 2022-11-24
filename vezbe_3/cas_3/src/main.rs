// Ownership
fn main() { 
    // prikaz vlasnistva na primeru prostih tipova i Stringa
    // prosti tipovi - poznata velicina memorije koju zauzimaju - cuvaju se na stack/u 
    // String [ovde samo primer, mogu da budu i drugi tipovi] - promenljive velicine - cuvaju se na heap/u
    
    //string literal 
    let lit_s = "hello"; // ovo je string literal, ne moze da menja velicinu, hardkodovan je
                               // cuva se na stack/u 
                               
    let str = String::from("Hello"); // String, promenljiv, mozemo da ga menjamo 
                                             // cuva se na heap/u 

    println!("String literal -> {}", lit_s);
    println!("String -> {}", str);

    println!("---------------------------------");

    // interakcija vise promenljivih
    //1. move
    //prosti tipovi
    let lit_str = lit_s; 
    println!("Literal str - {}, literal str_2 - {}", lit_s, lit_str ); // posto su u pitanju prosti tipovi koji se cuvaju na stack/u ovo ce da radi, jer dolazi do kopiranja vrednosti 
    
    //String
    let str2 = str; //prenos vlasnistva sa str na str2, str vise ne vazi
    // println!("String  1 - {}, string 2 - {}", str, str2); // ovo nece raditi [zato je zakomentarisano], zato sto promenljiva str ovde vise ne vazi
                                                          // kada vrednosti koje se cuvaju na heap/u dodelimo nekoj drugoj promenljivoj onda one prestaju da vaze
    

    //2. clone 
    //String - ako zelimo da i posle dodele vrednosti str vazi i dalje onda cemo uraditi clone
    // clone - pravi se kopija podataka na heap/u i nova promenljiva pokazuje na kopirane podatke 
    let s3 = String::from("hi");
    let str2 = s3.clone(); // s3 i dalje vazi posto je uradjeno kloniranje podataka, tako da nije doslo do prenosa vlasnistva 
    println!("String  2 - {}, string 3 - {}", str2, s3); //podaci se kopiraju tako da ce ovo da radi 


    println!("---------------------------------");
    //PROSLEDJIVANJE PROMENLJIVIH NEKOJ FUNKCIJI RADI ISTO KAO I DODELA VREDNOSTI
    takes_ownership(s3); // s3 prosledjujemo funkciji, menja vlasnika, tako da ovde prestaje da vazi za ovaj opseg i ne moze vise da se koristi posle poziva funkcije

    let x = 4;
    makes_copy(x); // ovde funkciji prosledjujemo promenljivu koja je prostog tipa, tako da dolazi do kopiranja vrednosti, pa x mozete da koristite i posle poziva funkcije
    println!("Main [posle poziva make_copy f-je] x - {}", x);



}

fn takes_ownership(s: String){
    println!("--------------------------------------------------------");
    // doslo je do prenosa vlasnistva nad promenljivom s, sada je vlasnik ova funkcija, a ne main
    println!("Funkcija {}", s);
}   //s ovde izlazi iz opsega i vise ne vazi 

fn makes_copy(x: i32){
    println!("--------------------------------------------------------");
    // x je kopirano i vazi od pocetka do kraja funkcije 
    println!("Funkcija make_copy x - {}", x);
}