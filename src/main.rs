// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
#[derive(Debug)]
enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
    Romer,
}

#[derive(Debug)]
struct SlopeIntercpet {
    slope: f64,
    y_intercept: f64,
}

impl Scale {
    const REF_SCALE: Scale = Scale::Kelvin;
    const fn get_slope_intercept(&self) -> SlopeIntercpet {
        match *self {
            Scale::Kelvin => SlopeIntercpet {
                slope: 1.0,
                y_intercept: 0.0,
            },
            Scale::Celsius => SlopeIntercpet {
                slope: 1.0,
                y_intercept: 273.15,
            },
            Scale::Fahrenheit => SlopeIntercpet {
                slope: 0.5555555555555556, // 5.0/9.0
                y_intercept: 255.37222222222223, // 459.67*5.0/9.0
            },
            Scale::Romer => SlopeIntercpet {
                slope: 1.9047619047619047, // 40.0/21.0
                y_intercept: 258.8642857142857, // 273.15-7.5*40.0/21.0
            },
        }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
struct Temperature {
    value: f64,
    scale: Scale,
}

impl Temperature {
    fn convert_to_ref_scale(&self) -> Temperature {
        let slope_intercept = self.scale.get_slope_intercept();
        Temperature {
            value: self.value * slope_intercept.slope + slope_intercept.y_intercept,
            scale: Scale::REF_SCALE,
        }
    }
    
    fn convert_to(&self, new_scale: Scale) -> Temperature {
        let slope_intercept = new_scale.get_slope_intercept();
        let ref_temp = self.convert_to_ref_scale();
        Temperature {
            value: ( ref_temp.value - slope_intercept.y_intercept ) / slope_intercept.slope,
            scale: new_scale,
        }
    }
}

// fn display_conversion(scale_a: fn() -> SlopeIntercpet, scale_b: fn() -> SlopeIntercpet, val_a: f64) {
//     let val_b = convert(scale_a(), scale_b(), val_a);
//     println!("{val_a:.2} {:10} is {val_b:.2} {:10}", scale_a().name, scale_b().name);
// }

fn main() {
    println!("\nWelcome to the temperature converter demo!\n");
    
    let temp = Temperature { value: 25.0, scale: Scale::Celsius };
    println!("Current {:?}\n", temp);
    println!("The reference scale is {:?}\n", Scale::REF_SCALE);
    println!("{:?} to {:?} {:?}\n", temp.scale, Scale::REF_SCALE, temp.scale.get_slope_intercept());
    let demo_scale = Scale::Kelvin;
    println!("{:?} to {:?} {:?}\n", demo_scale, Scale::REF_SCALE, demo_scale.get_slope_intercept());
    
    println!("Let's convert the current temperature to various scales:");
    println!("{:>10.2} {:?} ", temp.convert_to_ref_scale().value, Scale::REF_SCALE, );
    println!("{:>10.2} {:?} ", temp.convert_to(Scale::Celsius).value, Scale::Celsius, );
    println!("{:>10.2} {:?} ", temp.convert_to(Scale::Fahrenheit).value, Scale::Fahrenheit, );
    println!("{:>10.2} {:?} ", temp.convert_to(Scale::Romer).value, Scale::Romer, );
}