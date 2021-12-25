// GENERICS

struct Point<T, K> {
    x: T,
    y: T,
    city: K,
}

fn main() {
    let point_a = Point {
        x: 0.5,
        y: 12.5,
        city: "Miami",
    };

    let point_b = Point {
        x: 5.5,
        y: 12.0,
        city: "Orlando",
    };

    calculate_area(point_a, point_b);
}


fn calculate_area<T, K>(point_a: Point<T, K>, point_b: Point<T, K>) {
}