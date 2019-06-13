struct Cacher<T>
    where T: Fn(i32) -> i32
{
    call: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
    where T: Fn(i32) -> i32
{
    fn new(call: T) -> Cacher<T> {
        Cacher{
            call,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.call)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Counter {
    count: u32
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count > 6 {
            None
        } else {
            Some(self.count)
        }
    }
}

impl Counter {
    fn new() -> Self {
        Counter{ count: 0 }
    }
}

fn main() {
    let mut ca = Cacher::new(|a| a+1);
    let v1 = ca.value(4);
    let v2 = ca.value(8);
    println!("{}, {}", v1, v2);

    let con = Counter::new();
    for c in con.into_iter() {
        println!("{}", c);
    }
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)|a*b)
        .filter(|x| x%3 == 0)
        .sum();
    println!("{}", sum);
}


