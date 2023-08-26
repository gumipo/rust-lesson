struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        return PointAnother {
            x: self.x,
            y: other.y,
        };
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut lergest = number_list[0];

    // for number in number_list {
    //     if number > lergest {
    //         lergest = number
    //     }
    // }

    // let n = lergest_i32(number_list);

    // println!("{}", n);

    println!("{}", lergest(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];

    println!("{}", lergest(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = PointAnother { x: 11.2, y: 2 };
    let p3 = PointAnother { x: "Rust", y: '2' };

    let p4 = p2.mixup(p3);
    println!("{} {}", p4.x, p4.y);
}

fn lergest_i32(list: Vec<i32>) -> i32 {
    let mut lergest = list[0];

    for item in list {
        if item > lergest {
            lergest = item
        }
    }
    return lergest;
}

fn lergest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut lergest = list[0];

    for item in list {
        if item > lergest {
            lergest = item
        }
    }
    return lergest;
}
