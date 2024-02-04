use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> serde_json::Result<()> {
    let point = Point { x: 0, y: 0 };
    let point_str = serde_json::to_string(&point)?;
    let point2: Point = serde_json::from_str(&point_str)?;

    println!("{point:?} == {point2:?}");

    Ok(())
}
