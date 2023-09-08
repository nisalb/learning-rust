use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<P, Q>(self, other: Point<P, Q>) -> Point<T, Q> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    fn dist(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest i32 = {result}");

    let number_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&number_list);
    println!("largest = {result}");

    let _a = Point { x: 3, y: 4 };
    let b = Point { x: 2.3, y: 4.5 };

    let c = Point { x: 3, y: 4.0 };

    println!("p.x = {}, p.y = {}", c.x(), c.y());

    println!("distance to b = {}", b.dist());

    let k = Point { x: 3, y: 4 };
    let l = Point { x: "Hello", y: 'c' };

    let m = k.mixup(l);
    println!("m = {:?}", m);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
