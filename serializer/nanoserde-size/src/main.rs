use nanoserde::{DeJson, DeJsonErr, SerJson};

#[derive(SerJson, DeJson, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), DeJsonErr> {
    let point = Point { x: 0, y: 0 };
    let point_str = SerJson::serialize_json(&point);
    let point2: Point = DeJson::deserialize_json(&point_str)?;

    println!("{point:?} == {point2:?}");

    Ok(())
}
