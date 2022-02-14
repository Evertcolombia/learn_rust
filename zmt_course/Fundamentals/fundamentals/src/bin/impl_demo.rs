// impl keyword allow us implement functionality
// on specific enumerations and structs

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing(degrees: f64) -> Self {
        Self { degrees_f: degrees }
    }

    fn boling() -> Self {
        Self { degrees_f: 213.0 }
    }

    fn show_temp(&self) {
        println!("{:?} - degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing(32.4);
    cold.show_temp();

    let boiling = Temperature::boling();
    boiling.show_temp();
}