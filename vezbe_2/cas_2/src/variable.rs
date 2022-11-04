use std::any::type_name;
use std::io;
//funkcija za dodredjivanje tipa promenljive
fn type_of<T>(_: T) -> &'static str {
   type_name::<T>()
}

pub fn var_menu(){
    loop  {
        let mut option: String = String::new();
        println!("Meni");
        println!("1 - Tipovi podataka");
        println!("2 - Nepromenljive, promenljive i konstante");
        println!("3 - Preklapanje promenljive");
        println!("4 - Preklapanje promenljive vs mut promenjljiva");
        println!("0 - Glavni meni");

        // kod za unos podataka 
        io::stdin()
        .read_line(&mut option)
        .expect("Greska prilikom citanja podataka"); 

        let opt:i32 = option.trim().parse().expect("Unesena vrednost ne moze da se pretvori u broj.");

        match opt { 
            1 => def_variable(),
            2 => let_mut_const(),
            3 => var_shadowing(),
            4 => var_shadowing_vs_mut(),
            _other => break
        }
    }
}

fn def_variable(){

    // definisanje promenljive    
    //ukoliko je vrednost promenljive unapred poznata onda ne mora, a i moze da se navode njen tip
    let a = 3; 
    let x: i32 = 234;
    let f = 2.0;
    let f32: f32 = 2.4; //navedemo tip zato sto je podrazumevan tip f64;
    let s: String = String::from("hello");
    let s_lit = "Ovo je string literal.";
    //char => jedan karakter, koji se navodi u okviru ''
    let c = 'C';
    let smile: char = '😻';

    //boolean
    let b = false;


    println!("==============================");
    //ispis tipova promenljivih
    println!("{0: <30} | {1: <10}", "vrednost", "tip");
    println!("----------------------------------------");
    println!("{0: <30} | {1: <10}", a, type_of(a));
    println!("{0: <30} | {1: <10}", x, type_of(x));
    println!("{0: <30} | {1: <10}", f, type_of(f));
    println!("{0: <30} | {1: <10}", f32, type_of(f32));
    print!("{0: <30} | ", s);
    println!("{0: <10}", type_of(s));
    println!("{0: <30} | {1: <10}", s_lit, type_of(s_lit));
    println!("{0: <30} | {1: <10}", c, type_of(c));
    println!("{0: <30}| {1: <10}", smile, type_of(smile));
    println!("{0: <30} | {1: <10}", b, type_of(b));    
    println!("==============================");

}

fn let_mut_const(){
    let x = 80; 

    // x = x + 1; // ovo NE MOZE, posto su promenljive podrazumevano nepromenljive (ovo ce biti zakomentarisano posto se kod nece kompajlirati zbog ovoga)

    // Resenje: MUT PROMENLJIVE 
    let mut y = 120; 
    println!("x: {x}");
    
    println!("Unesite broj: ");
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Greska prilikom citanja podataka."); 

    let n:i32 = num.trim().parse().expect("Uneseni podatak ne moze da se pretvori u int");
    println!("y = {y}");
    println!("y + {} = {}", n, y + n);


    // const -> konstante, njihova vrednost ne moze da se menja i uvek mora da se navede tip
    const c: i32 = 3 + 89; 
    println!("const c -> {c}");

}

fn var_shadowing(){
    // preklapanje promenljivih je dozvoljeno -> definisanje promenljive sa istim imenom vise puta

    let x = 45;


    let x = x + 1; // videli smo malo pre da x = x + 1; nije dozvoljeno
                   // a ovo iznad moze, zato sto koristimo let pa je ovo zapravo definisanje nove x promenljive
                   // stara x promenljiva ovde prestaje da vazi

    { // blok - moze da se definise bilo gde, izlaskom iz bloka prestaju da vaze sve promenljive koje su u njemu kreirane
        let x = x * 20;
        println!("Vrednost promenljive x u bloku: {x}"); 
    }

    println!("Vrednost promenljive x izvan bloka: {x}");

} 

fn var_shadowing_vs_mut(){
    let s = "hello";
    println!("s = {s}");

    let s = s.len(); //preklapanje promenljive s, ovde ce s da bude int a pre je bila string
                    // preklapanje promenljiva dozvoljava da menjamo tip promenljive
    println!("s = {s}");


    let mut str = "hello"; // mut promenljivu mozete da menjate, ali ne mozete da joj promenite tip, a sa preklapanjem je to dozvoljeno
    println!("str = {str}");
    // str = str.len(); //ova linija ce da izazove GRESKU! - zato je zakomentarisana, (Predlog: otkomentarisati i videti sta se desava)
    println!("str = {str}");

}