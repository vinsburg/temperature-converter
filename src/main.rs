// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
#[derive(Debug)]
#[allow(dead_code)]
struct ScaleData {
    ratio: f64,
    del_x: f64,
    del_y: f64,
}

const KELVIN: ScaleData = ScaleData {
    ratio: 1.0,
    del_x: 0.0,
    del_y: 0.0,
};

const CELSIUS: ScaleData = ScaleData {
    ratio: 1.0,
    del_x: 273.15,
    del_y: 0.0,
};

#[derive(Debug)]
#[allow(dead_code)]
enum ScaleType {
    Kelvin,
    Celsius,
}

impl ScaleType {
    fn get_scale_data(&self) -> ScaleData {
        match self {
            ScaleType::Kelvin => KELVIN,
            ScaleType::Celsius => CELSIUS,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Temperature {
    value: f64,
    scale: ScaleType,
}

// fn fahr() -> ScaleData {
//     ScaleData {
//         name: String::from("Fahrenheit"),
//         ratio: 5.0/9.0,
//         del_x: 459.67,
//         del_y: 0.0,
//     }
// }

// fn rome() -> ScaleData {
//     ScaleData {
//         name: String::from("Rømer"),
//         ratio: 40.0/21.0,
//         del_x: -7.5,
//         del_y: 273.15,
//     }
// }

// fn convert(scale_a: ScaleData, scale_b: ScaleData, val_a: f64) -> f64 {
//     orig_to_dest(
//         scale_b,
//         dest_to_orig(
//             scale_a,
//             val_a,
//         ),
//     )
// }

// fn display_conversion(scale_a: fn() -> ScaleData, scale_b: fn() -> ScaleData, val_a: f64) {
//     let val_b = convert(scale_a(), scale_b(), val_a);
//     println!("{val_a:.2} {:10} is {val_b:.2} {:10}", scale_a().name, scale_b().name);
// }

fn main() {
    println!("KELVIN - {:?}", KELVIN);
    
    let temp = Temperature { value: 0.0, scale: ScaleType::Celsius };
    println!("Current temperature - {:?}", temp);
    println!("Current scale data - {:?}", temp.scale.get_scale_data())

    // println!("{temperature} Kelvin is {} base units", KELV.to_base_scale(temperature));
    // println!("{temperature} base units is {} Kelvin", KELV.from_base_scale(temperature));
}