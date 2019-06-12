use std::collections::HashMap;

enum Data {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let mut v = Vec::<i32>::new();
    let v1 = vec![1,2,3,4];
    v.push(1);
    v.push(2);

    println!("Hello, world!, {:?}, {:?} {}",v.get(1), v.get(2), &v[0]);
    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 1;
    }
    let low = vec![Data::Int(1), Data::Float(5.7), Data::Text(String::from("Lome"))];

    let mut scores = HashMap::new();
    scores.insert(String::from("blo"), 87);
    scores.insert(String::from("blu"), 67);

    let team = vec![String::from("lome"), String::from("loey")];
    let so = vec![10, 30];
    let score: HashMap<_, _> = so.iter().zip(team.iter()).collect();
    println!("{:?}", score.get(&10));
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("lome")).or_insert(98);

    let text = "my name is gaoyujie, what you name is?";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //or_insert 方法事实上会返回这个键的值的一个可变引用
        *count += 1;
    }
    println!("{:?}", map);

}
