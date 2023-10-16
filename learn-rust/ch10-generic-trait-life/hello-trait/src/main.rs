trait Drawable {
    // 绘制图形并返回一个字符串描述
    fn draw(&self) -> String;
    // 获取图形的面积
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side_length: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Drawing a circle with radius {}.", self.radius)
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Square {
    fn draw(&self) -> String {
        format!("Drawing a square with side length {}.", self.side_length)
    }

    fn area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side_length: 4.0 };

    let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(circle), Box::new(square)];

    for shape in shapes {
        println!("{}", shape.draw());
        println!("Area: {}", shape.area());
    }
}
