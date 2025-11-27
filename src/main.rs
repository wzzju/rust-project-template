// region:    --- Modules

use {{project-name}}::{Result, utils};

use std::collections::{LinkedList, VecDeque};

// endregion: --- Modules

fn main() -> Result<()> {
    let (squares, cubes, tesseracts): (Vec<_>, VecDeque<_>, LinkedList<_>) =
        (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4))).collect();
    println!("{squares:?}");
    println!("{cubes:?}");
    println!("{tesseracts:?}");

    let c = utils::add(1, 2);
    println!("1 + 2 = {c}");

    let msg = utils::greet(None);
    println!("Greet: {msg}");

    Ok(())
}
