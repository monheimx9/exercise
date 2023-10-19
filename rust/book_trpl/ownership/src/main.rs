fn main() {
    let s = "hello";
    let mut p = String::from("Hey mom");
    p.push_str(", what's up?");
    println!("{}", p);
    println!("{}", s);
    let i = p.clone();
    println!("{} {}", p, i);
}
