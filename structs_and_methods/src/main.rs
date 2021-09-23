mod lib;

use lib::{Figure, Circle, Rect, Point};

fn main() {
    let zero_point = Point { x: 0.0 , y: 0.0 };

    let examples = [
        Figure::Circle(Circle { center: Point {x: -1.0, y: -1.0}, radius: 1.0 }),
        Figure::Circle(Circle { center: Point {x: -1.0, y: -1.0}, radius: 2.0 }),
        Figure::Rect(Rect { x: 1.0, y: 2.0, width: 12.0, height: 15.0 }),
        Figure::Rect(Rect { x: -1.0, y: -2.0, width: 100.0, height: 15.0 })
    ];

    for x in examples {
        println!("{:?}, area = {1}, contains zero - {2}", x, x.area(), x.contains(&zero_point))
    }
}