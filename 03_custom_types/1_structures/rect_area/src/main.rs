// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn square(point: &Point, side: f32) -> Rectangle {
    let (top_left_x, top_left_y) = destructure_point(point);
    let bottom_right = Point {
        x: top_left_x + side,
        y: top_left_y - side,
    };
    Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right,
    }
}

const fn destructure_point(point: &Point) -> (f32, f32) {
    let Point { x, y } = point;
    (*x, *y)
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left,
        bottom_right,
    } = rect;

    let (left_x, top_y) = destructure_point(top_left);
    let (right_x, bottom_y) = destructure_point(bottom_right);

    (right_x - left_x) * (top_y - bottom_y)
}

fn main() {
    let top_left = Point { x: -1.75, y: 2.37 };
    let bottom_right = Point { x: 2.25, y: -7.52 };

    let rect1 = Rectangle {
        top_left,
        bottom_right,
    };
    println!("The area of rectangle is {}", rect_area(&rect1));
}
