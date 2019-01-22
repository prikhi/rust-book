fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point2 { x: 5, y: 4.0 };

    println!("integer.x = {}", integer.x());
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            // needs trait, which is covered in next section
            largest = item;
        }
    }

    largest
}

// generic structs
struct Point<T> {
    x: T,
    y: T,
}
struct Point2<T, U> {
    x: T,
    y: U,
}
// generic methods
impl<T> Point<T> {
    fn x(&self) -> T {
        self.x
    }
}
// type specific methods
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T, U> Point2<T, U> {
    // can still use arbitrary generics in method functions
    fn mixup<X, Y>(&self, other: &Point2<X, Y>) -> Point2<T, Y> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// generic enums
enum Maybe<T> {
    Just(T),
    Nothing,
}
enum Either<T, E> {
    Right(T),
    Left(E),
}
