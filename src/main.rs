extern crate absal;

fn main() {
    // Parses the following λ-program:
    //   two = λf. λx. f (f x)
    //   exp = λn. λm. m n
    //   exp two two
    let (stats, code) = absal::reduce("/ #f #x /f /f x #f #x /f /f x");
    println!("{:?}", stats);
    println!("{}", code);
}
