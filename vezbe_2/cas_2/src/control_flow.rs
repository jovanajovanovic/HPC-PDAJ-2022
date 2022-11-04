pub fn control_flow_menu(){
    // ova konstrukcija je vec koriscena na vise mesta
    // loop - petlja koja se izvrsava beskonacno puta, moze da bude prekinuta sa CTRL+C ili upotrebom break
    // ovde je prekinuta sa break, koji se poziva kada se ne unese jedna od ponudjenih opcija menija
    loop  {
        let mut option: String = String::new();
        println!("Meni");
        println!("1 - if");
        println!("2 - loop");
        println!("0 - Glavni meni");

        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect("Greska prilikom citanja podataka"); 

        let opt:i32 = option.trim().parse().expect("Unesena vrednost ne moze da se pretvori u broj.");

        match opt { 
            1 => if_stmt(),
            2 => fun_with_return_value(),
            _other => break
        }
    }
}

fn if_stmt(){
    // primer: provera da li je random broj veci ili manji od 10
