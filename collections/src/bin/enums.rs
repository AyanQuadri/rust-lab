#[derive(Debug)]
enum Directions {
    North,
    East,
    South,
    West,
}

fn main() {
    let dir1 = Directions::North;
    let dir2 = Directions::South;
    let dir3 = Directions::West;
    let dir4 = Directions::East;

    println!("{:?}", dir1);
    println!("{:?}", dir2);
    println!("{:?}", dir3);
    println!("{:?}", dir4);
}
