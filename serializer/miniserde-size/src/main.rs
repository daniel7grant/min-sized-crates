use miniserde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> miniserde::Result<()> {
    let point: Point = Point { x: 0, y: 0 };
    let point_str = json::to_string(&point);
    let point2: Point = json::from_str(&point_str)?;

    println!("{point:?} == {point2:?}");

    Ok(())
}
