use example_lib::print::*; // uvezemo sve iz modula print koji se nalazi u example_lib biblioteci;
                           // ovo je bez upotrebe pub use/a 

use example_lib::add; // jednostavnije uvezemo sum funkciju, upotrebom pub use u biblioteci

fn main() {
    print_hi(); //poziv funkcije iz biblioteke

    println!("{}", add(3, 5));
}
