mod days;
mod surreal;

fn main() {
    let mut numbers = days::Numbers::new();
    for _ in 0..3 {
        numbers.next_day();
    }
    for i in 0..numbers.len() {
        println!("{}: {:?}", i, numbers.get_ordered(i));
    }
}
