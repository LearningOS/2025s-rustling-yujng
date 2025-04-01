// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // let y = Some(&{ Point { x: 100, y: 200 } });
    // if let Some(p) = y {

    // }
    let y = Some(Point { x: 100, y: 200 });
    if let Some(ref p) = y {
        println!("Co-ordinates are {},{} ", p.x, p.y)
    };
    y;
}
