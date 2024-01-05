use crate::surreal::Surreal;
use std::rc::Rc;

pub struct Numbers {
    numbers: Vec<Rc<Surreal>>,
}

impl Numbers {
    pub fn new() -> Numbers {
        Numbers {
            numbers: vec![Rc::new(Surreal::zero())],
        }
    }

    pub fn days(&self) -> usize {
        self.numbers.len().count_ones() as usize
    }

    pub fn get(&self, day: usize, index: usize) -> &Surreal {
        assert!(day < self.days(), "day {} is out of range", day);
        assert!(
            index < (1 << day),
            "index {} is out of range for day {}",
            index,
            day
        );
        &self.numbers[(1 << day) - 1 + index]
    }

    pub fn get_ordered(&self, index: usize) -> Rc<Surreal> {
        assert!(
            index < self.numbers.len(),
            "index {} is out of range",
            index
        );
        let day_height = index.trailing_ones() + 1;
        let day = self.days() - day_height as usize;

        let idx = (1 << day) - 1 + index >> day_height;

        self.numbers[idx].clone()
    }

    pub fn next_day(&mut self) {
        let mut new_numbers = Vec::with_capacity(self.numbers.len() + 1);
        let range = 0..(self.numbers.len() as i64);
        for i in 0..=(self.numbers.len() as i64) {
            let first = i - 1;
            let second = i;
            new_numbers.push(Rc::new(Surreal::new(
                if range.contains(&first) {
                    Some(self.get_ordered(first as usize))
                } else {
                    None
                },
                if range.contains(&second) {
                    Some(self.get_ordered(second as usize))
                } else {
                    None
                },
            )));
            println!("{}", self.numbers.last().unwrap());
        }
        self.numbers.append(&mut new_numbers);
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }
}
