// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
#[derive(Clone, Copy, Debug)]
enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
    Romer,
}

#[derive(Debug)]
struct SlopeIntercept {
    slope: f64,
    y_intercept: f64,
}

impl Scale {
    const REF_SCALE: Scale = Scale::Kelvin;
    const fn get_slope_intercept(&self) -> SlopeIntercept {
        match *self {
            Scale::Kelvin => SlopeIntercept {
                slope: 1.0,
                y_intercept: 0.0,
            },
            Scale::Celsius => SlopeIntercept {
                slope: 1.0,
                y_intercept: 273.15,
            },
            Scale::Fahrenheit => SlopeIntercept {
                slope: 0.5555555555555556, // 5.0/9.0
                y_intercept: 255.37222222222223, // 459.67*5.0/9.0
            },
            Scale::Romer => SlopeIntercept {
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

fn main() {
    println!("\nWelcome to the temperature converter demo!\n");
    
    let temp = Temperature { value: 25.0, scale: Scale::Celsius };
    println!("Current {:?}\n", temp);
    println!("The reference scale is {:?}\n", Scale::REF_SCALE);
    println!("{:?} to {:?} {:?}", temp.scale, Scale::REF_SCALE, temp.scale.get_slope_intercept());
    println!("Current temperature in {:?} scale is {:?}°\n", Scale::REF_SCALE, temp.convert_to_ref_scale().value);
    let demo_scale = Scale::Kelvin;
    println!("{:?} to {:?} {:?}", demo_scale, Scale::REF_SCALE, demo_scale.get_slope_intercept());
    
    println!("\nLet's convert the current temperature to various scales:");
    for scale in [Scale::Kelvin, Scale::Celsius, Scale::Fahrenheit, Scale::Romer] {
        println!("T = {:>6.2}° {:?} ", temp.convert_to(scale).value, scale, );
    }
}