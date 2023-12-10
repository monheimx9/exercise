use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut m: HashMap<usize, usize> = HashMap::new();
    let i = fib(50, &mut m);
    println!("{i}")
}

fn fib(n: usize, m: &mut HashMap<usize, usize>) -> usize {
    if m.get(&n).is_some() {
        return *m.get(&n).unwrap();
    }
    if n <= 2 {
        return 1;
    } else {
        let i = fib(n - 1, m) + fib(n - 2, m);
        m.insert(n, i);
        return *m.get(&n).unwrap();
    }
}
