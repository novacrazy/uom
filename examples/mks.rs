//! Example showing how to create a custom system of quantities.

#[macro_use]
extern crate uom;

use length::{foot, meter};

fn main() {
    let l1 = f32::Length::new::<meter>(100.0);

    println!(
        "{:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        l1.get::<foot>(),
        foot::abbreviation()
    );
}

#[macro_use]
mod length {
    quantity! {
        /// Length (base unit meter, m<sup>1</sup>).
        quantity: Length; "length";
        /// Length dimension, m<sup>1</sup>.
        dimension: Q<
            P1,  // length
            Z0,  // mass
            Z0>; // time
        units {
            @meter: 1.0E0; "m", "meter", "meters";
            @foot: 3.048E-1; "ft", "foot", "feet";
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        /// Mass (base unit kilogram, kg<sup>1</sup>).
        quantity: Mass; "mass";
        /// Mass dimension, kg<sup>1</sup>.
        dimension: Q<
            Z0,  // length
            P1,  // mass
            Z0>; // time
        units {
            @kilogram: 1.0; "kg", "kilogram", "kilograms";
        }
    }
}

#[macro_use]
mod time {
    quantity! {
        /// Time (base unit second, s<sup>1</sup>).
        quantity: Time; "time";
        /// Time dimension, s<sup>1</sup>.
        dimension: Q<
            Z0,  // length
            Z0,  // mass
            P1>; // time
        units {
            @second: 1.0; "s", "second", "seconds";
        }
    }
}

system! {
    quantities: Q {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
    }

    units: U {
        mod length::Length,
        mod mass::Mass,
        mod time::Time,
    }
}

mod f32 {
    mod s {
        pub use *;
    }

    Q!(f32::s, f32);
}
