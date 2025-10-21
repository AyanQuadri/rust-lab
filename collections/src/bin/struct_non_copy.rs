struct Owner {
    name: String,
}

struct House {
    owner: Owner,
    rooms: u32,
}

fn main() {
    let owner1 = Owner {
        name: String::from("Alice"),
    };

    let house1 = House {
        owner: owner1, // The ownership will be moved to this
        rooms: 3,
    };

    /*
     * Now you can't access owner1 values because it has been
     * transferred to owner in the house1
     * */

    println!("House owner: {}", house1.owner.name);
    println!("Number of rooms: {}", house1.rooms);
}
