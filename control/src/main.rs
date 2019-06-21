fn main() {
    // if
    let number = 10;
    if number > 5 {
        println!("the number is bigger 5");
    } else if number > 20 {
        println!("the number is bigger 20");
    } else {
        println!("the number is bigger 5 less 20");
    }

    let num = if number > 5 {
        6
    } else {
        3
    };

    let mut o = 0;
    let mut numb = loop {
        if o > 10 {
            break o*8
        }
        o += 1;
    };

    while numb < 100 {
        numb += 1;
    }

    let vep = [1,2,3,4,5,6];
    for i in vep.iter() {
        println!("i {}", i);
    }

    for s in (1..=4).rev() {
        println!("s: {}",s);
    }
}
