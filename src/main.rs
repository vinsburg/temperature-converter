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

const FAHRENHEIT: ScaleData = ScaleData {
    ratio: 5.0/9.0,
    del_x: 459.67,
    del_y: 0.0,
};

const ROMER: ScaleData = ScaleData {
    ratio: 40.0/21.0,
    del_x: -7.5,
    del_y: 273.15,
};

#[derive(Debug)]
#[allow(dead_code)]
enum ScaleType {
    Kelvin,
    Celsius,
    Fahrenheit,
    Romer,
}

impl ScaleType {
    fn get_scale_data(&self) -> ScaleData {
        match self {
            ScaleType::Kelvin => KELVIN,
            ScaleType::Celsius => CELSIUS,
            ScaleType::Fahrenheit => FAHRENHEIT,
            ScaleType::Romer => ROMER,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Temperature {
    value: f64,
    scale: ScaleType,
}

impl Temperature {
    fn convert_to_kelvin(&self) -> Temperature {
        let scale_data = self.scale.get_scale_data();
        Temperature {
            value: (self.value + scale_data.del_x) * scale_data.ratio + scale_data.del_y,
            scale: ScaleType::Kelvin,
        }
    }
    
    fn convert_to(&self, scale: ScaleType) -> Temperature {
        let scale_data = scale.get_scale_data();
        let kelvin_temp = self.convert_to_kelvin();
        Temperature {
            value: (kelvin_temp.value - scale_data.del_y) / scale_data.ratio - scale_data.del_x,
            scale: scale,
        }
    }
}

// fn display_conversion(scale_a: fn() -> ScaleData, scale_b: fn() -> ScaleData, val_a: f64) {
//     let val_b = convert(scale_a(), scale_b(), val_a);
//     println!("{val_a:.2} {:10} is {val_b:.2} {:10}", scale_a().name, scale_b().name);
// }

fn main() {
    println!("Kelvin {:?}", KELVIN);
    
    let temp = Temperature { value: 25.0, scale: ScaleType::Celsius };
    println!("Current {:?}", temp);
    println!("Current {:?}", temp.scale.get_scale_data());
    println!("Baseline {:?}", temp.convert_to_kelvin());
    println!("Roundtrip {:?}", temp.convert_to(ScaleType::Celsius));
    println!("Alternative {:?}", temp.convert_to(ScaleType::Fahrenheit));
    println!("Other Alternative {:?}", temp.convert_to(ScaleType::Romer));
}