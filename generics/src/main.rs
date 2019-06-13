struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summary {
    fn summary(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{} , by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct  Tweet {
    pub username: String,
    pub retweet: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{} get {}", self.username, self.content)
    }
}

//fn newsum(data: &Vec<String>) -> impl Summary {
//        if data.len() > 3 {
//            NewArticle{
//                headline: data[0].clone(),
//                location: data[1].clone(),
//                author: data[2].clone(),
//                content: data[3].clone(),
//            }
//        } else {
//           Tweet {
//                username: data[0].clone(),
//                retweet: data[1].clone(),
//                content: data[2].clone(),
//            }
//        }
//}

fn lagest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn longest<'a>(re: &'a str, le: &'a str) -> &'a str {
    if re.len() > le.len() {
        re
    } else {
        le
    }
}

fn main() {
    let s = Point{ x: 1, y: 3.0};
    let p = Point{ x: "lome", y : String::from("LOME")};
    let t = s.mixup(p);
    println!("x: {}, y: {}", t.x, t.y);
    let art = NewArticle{
        headline: String::from("rust"),
        location: String::from("HeNan"),
        author: String::from("lome"),
        content: String::from("this is a rust language!"),
    };
    println!("{}", art.summary());

    let twe = Tweet {
        username: String::from("lome"),
        retweet: String::from("yes"),
        content: String::from("this is a content!"),
    };
    println!("{}", twe.summary());

    let sp = vec![String::from("rust"), String::from("HeNan"), String::from("lome"), String::from("this is a rust language!")];
//    let sum = newsum(&sp);
//    println!("{}", sum.summary());
    let spd = vec![1,3,4,2,6,5];
    let la = lagest(&spd);
    println!("{}", la);

    let s1 = String::from("lome");
    let s2 = String::from("lalalalala");
    let ap = longest(&s1, &s2);
    println!("{}", ap);
}
