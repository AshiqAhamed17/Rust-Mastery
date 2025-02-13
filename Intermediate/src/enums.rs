
enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

fn calculate_area(shape: Shape) -> f32 {
    let ans = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(l, b) => {
            l * b
        },
    };
    ans // Same as => return ans;
}

pub fn run() {
    
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rect = Shape::Rectangle(2.0, 3.0);

    println!("Circle: {}", calculate_area(circle));
    println!("Square: {}", calculate_area(square));
    println!("Rectangle: {}", calculate_area(rect));

    //println!("Circel: {}", circle);
}