// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calculator(&self, number: Option<i32>) -> i32;
}

struct Square {
    top: i32,
    left: i32,
    right: i32,
    bottom: i32,
}

impl Perimeter for Square {
    fn calculator(&self, number: Option<i32>) -> i32 {
        return number
            .map(|num| {
                let top = self.top * num;
                let left = self.left * num;
                let right = self.right * num;
                let bottom = self.bottom * num;
                let result = top + left + right + bottom;
                return result;
            })
            .unwrap();
    }
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Perimeter for Triangle {
    fn calculator(&self, number: Option<i32>) -> i32 {
        let result = self.a + self.b + self.c;
        result
    }
}

fn calculator(shape: impl Perimeter, number: Option<i32>) {
    shape.calculator(number);
    match &shape {
        Sqaure => println!("Sqaure's sides: {:?}", shape.calculator(number)),
        Triangle => println!("Triangle's sides: {:?}", shape.calculator(number)),
        _ => println!("Unknown shape."),
    }
}

fn main() {
    let square = Square {
        top: 3,
        left: 6,
        right: 9,
        bottom: 12,
    };
    calculator(square, Some(4));

    let triangle = Triangle { a: 1, b: 2, c: 3 };
    calculator(triangle, None);
}
