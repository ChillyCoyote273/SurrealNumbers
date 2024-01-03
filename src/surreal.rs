#[derive(Debug, Clone)]
struct Surreal {
    left: Vec<Surreal>,
    right: Vec<Surreal>,
}

impl Surreal {
    fn new() -> Surreal {
        Surreal {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}