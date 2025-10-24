enum Trafficlight {
    Red,
    Yellow,
    Green,
}

impl Trafficlight {
    fn duration(&self) -> u8 {
        match self {
            Trafficlight::Red => 60,
            Trafficlight::Yellow => 3,
            Trafficlight::Green => 30,
        }
    }
}

fn main() {
    let red_light = Trafficlight::Red;
    let yellow_light = Trafficlight::Yellow;
    let green_light = Trafficlight::Green;

    println!("Red light duration seconds: {}", red_light.duration());
    println!("Yellow light duration seconds: {}", yellow_light.duration());
    println!("Green light duration seconds: {}", green_light.duration());
}
