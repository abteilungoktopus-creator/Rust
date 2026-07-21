use std::io;
fn main() { 
  loop { 
    let mut a1 = String::new();
    println!("Was ist deine erste Zahl?");
    io::stdin().read_line(&mut a1).expect("?");
    let b1: f32 = a1.trim().parse().expect("keine Zahl");

    let mut a2 = String::new();
    println!("Was ist deine zweite Zahl?");
    io::stdin().read_line(&mut a2).expect("?");
    let b2: f32 = a2.trim().parse().expect("keine Zahl");

    println!("welche Operation soll durchgeführt werden?");
    let mut a3 = String::new();
    io::stdin().read_line(&mut a3).expect("?");
    let mut b3 = String::new();
    let b3 = a3.trim();

    if b3 == "+" {
        let d1 = b1 + b2;
        println!("Die Antwort lautet: {}",d1);
} else if b3 == "-" {
    let d1 = b1 - b2;
    println!("Die Antwort lautet: {}", d1);
}else if b3 == "*" {
    let d1 = b1*b2;
    println!("Die Antwort lautet: {}", d1);
} else {
    let d1: f32 = b1/b2;
    println!("Die Antwort lautet: {}", d1);
}
    println!("Fertig? y/n");
    let mut e = String::new();
    io::stdin().read_line(&mut e).expect("?");
    let e1 = e.trim();  
    let e2 = "y";
    if e1 == e2 {
        break;
    }
}
}
