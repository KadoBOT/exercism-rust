use std::cell::RefCell;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
    num: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec![(a, b)],
            num: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.num
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        let is_same = self.factors.iter().any(|(m, n)| &a == n && &b == m);
        if !is_same {
            self.factors.push((a, b));
            self.num = a * b;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }
}

fn rev_num(n: u64) -> u64 {
    n.to_string()
        .chars()
        .enumerate()
        .map(|(idx, ch)| ch.to_digit(10).unwrap() as u64 * 10u64.pow(idx as u32))
        .sum()
}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal = RefCell::new(Palindrome::default());
    let mut max_pal = RefCell::new(Palindrome::default());

    for n in min..=max {
        for m in min..=max {
            let num = n * m;
            if num > min_pal.borrow().value() && !min_pal.borrow().is_empty() {
                break;
            }
            if num == rev_num(num) {
                if num == min_pal.borrow().value() {
                    min_pal.borrow_mut().insert(n, m);
                } else {
                    min_pal = RefCell::new(Palindrome::new(n, m));
                }
            }
        }
    }

    for n in (min..=max).rev() {
        for m in (min..=max).rev() {
            let num = n * m;
            if num < max_pal.borrow().value() && !max_pal.borrow().is_empty() {
                break;
            }
            if num == rev_num(num) {
                if num == max_pal.borrow().value() {
                    max_pal.borrow_mut().insert(m, n);
                } else {
                    max_pal = RefCell::new(Palindrome::new(m, n));
                }
            }
        }
    }

    if min_pal.borrow().is_empty() {
        return None;
    }

    Some((min_pal.into_inner(), max_pal.into_inner()))
}
