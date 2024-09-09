struct Coordinate <T> {
    x: T,
    y: T
}

struct CoordinateTwo <T, X> {
    x: T,
    y: X
}

macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    }
}



fn main() {
    // let one = CoordinateTwo{x: 50, y: 500};
    // let two = CoordinateTwo{x: 5.6, y: 500};
    // let three = CoordinateTwo{x: 5.6, y: 50};

    // let one = CoordinateTwo{x: 50, y: 500};
    // let two = CoordinateTwo{x: 5.6, y: 500};
    // let three = CoordinateTwo{x: 5.6, y: 50};

    let mut x = String::from("test");
    capitalize!(x);
    println!("{}", x);
}
