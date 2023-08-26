#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn gen_point(x: X1, y: Y1) -> Point<X1, Y1> {
        let generated_point = Point { x: x, y: y };
        generated_point
    }
    fn modifier_x(&mut self, x: X1) -> &Point<X1, Y1> {
        self.x = x;
        self
    }
    fn mix_up<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        // self.y = other.y;
        Point {
            x: self.x,
            y: other.y,
        }
    }
    // fn distance_from_origin(&self)-> f32{
    //      (self.x.powi(2)+ self.y.powi(2)).sqrt()
    // }
}
fn main() {
    // let p1 = Point { x: 4, y: 4.0 };
    let mut p1 = Point::gen_point(1, 3.3);
    p1.modifier_x(4);
    let p2 = Point {
        x: 67.4,
        y: "niubi",
    };
    println!("{:#?}", &p1);
    let p_mixed =  p1.mix_up(p2);
    println!("{:#?}", p_mixed);
}

// fn gen_point(x: i32, y: i32) -> Point<i32, i32> {
//     Point { x: x, y: y }
// }
