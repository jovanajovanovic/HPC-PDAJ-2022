pub fn fun_menu(){
    loop  {
        let mut option: String = String::new();
        println!("Meni");
        println!("1 - Funkcija bez povratne vrednosti");
        println!("2 - Funkcija sa povratnom vrednoscu");
        println!("0 - Glavni meni");

        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect("Greska prilikom citanja podataka"); 

        let opt:i32 = option.trim().parse().expect("Unesena vrednost ne moze da se pretvori u broj.");

        match opt { 
            1 => fun_without_return_value(),
            2 => fun_with_return_value(),
            _other => break
        }
    }
}

fn fun_with_return_value() {
    //u okviru funkcije mozete da imate izraze i iskaze
    // iskazi - naredbe, kao sto su: 
    let a = 5; 
    println!("a = {a}");
    // izrazi - izracunava se do rezultujuce vrednosti i moze da bude sastavni deo iskaza
    let b = a + 3; // a + 3 => izraz
    println!("b = {b}");
}

fn fun_with_return_value() {
    println!("a + b = {}", sum(23, 56));
    println!("a + b = {}", sum_str(String::from("hello "), String::from("world")));
}

// parametri funkcije se navode u zagradama posle naziva funkcije, treba da se definise ime i tip parametra
//ako funkcija ima povratnu vrednost onda tip povratne vrednost treba da bude napisan odmah posle liste parametara
fn sum(a: i32, b:i32) -> i32{
    println!("{}",a);
    println!("{b}");
    a + b //ako nema kljucne reci return onda ce iz funkcije biti vracen poslednji izraz koji je napisan bez ; na kraju
}

fn sum_str(a: String, b: String) -> String{
    println!("{a}");
    println!("{b}");
    return a+b; //mozete i na ovaj nacin da vratite vrednost, ali onda na kraju mora da stoji ;
}