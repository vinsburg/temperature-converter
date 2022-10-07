// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
#[derive(Debug)]
enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
    Romer,
}

#[derive(Debug)]
struct ReferenceScaleData {
    ratio: f64,
    del_x: f64,
    del_y: f64,
    ref_scale: Scale,
}

impl Scale {
    const fn get_ref_scale_data(&self) -> ReferenceScaleData {
        let ref_scale = Scale::Kelvin;
        match *self {
            Scale::Kelvin => ReferenceScaleData {
                ratio: 1.0,
                del_x: 0.0,
                del_y: 0.0,
                ref_scale: ref_scale,
            },
            Scale::Celsius => ReferenceScaleData {
                ratio: 1.0,
                del_x: 273.15,
                del_y: 0.0,
                ref_scale: ref_scale,
            },
            Scale::Fahrenheit => ReferenceScaleData {
                ratio: 0.55555555555, // 5.0/9.0
                del_x: 459.67,
                del_y: 0.0,
                ref_scale: ref_scale,
            },
            Scale::Romer => ReferenceScaleData {
                ratio: 1.90476190476, // 40.0/21.0
                del_x: -7.5,
                del_y: 273.15,
                ref_scale: ref_scale,
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
        let ref_scale_data = self.scale.get_ref_scale_data();
        Temperature {
            value: (self.value + ref_scale_data.del_x) * ref_scale_data.ratio + ref_scale_data.del_y,
            scale: ref_scale_data.ref_scale,
        }
    }
    
    fn convert_to(&self, new_scale: Scale) -> Temperature {
        let ref_scale_data = new_scale.get_ref_scale_data();
        let ref_temp = self.convert_to_ref_scale();
        Temperature {
            value: (ref_temp.value - ref_scale_data.del_y) / ref_scale_data.ratio - ref_scale_data.del_x,
            scale: new_scale,
        }
    }
}

// fn display_conversion(scale_a: fn() -> ReferenceScaleData, scale_b: fn() -> ReferenceScaleData, val_a: f64) {
//     let val_b = convert(scale_a(), scale_b(), val_a);
//     println!("{val_a:.2} {:10} is {val_b:.2} {:10}", scale_a().name, scale_b().name);
// }

fn main() {
    println!("\nWelcome to the temperature converter demo!\n");
    let demo_scale = Scale::Kelvin;
    let demo_ref_scale_data = demo_scale.get_ref_scale_data();
    println!("{:?} {:?}", demo_scale, demo_ref_scale_data);
    
    
    let temp = Temperature { value: 25.0, scale: Scale::Celsius };
    println!("\nCurrent {:?}", temp);
    println!("{:?} {:?}", temp.scale, temp.scale.get_ref_scale_data());
    println!("Reference {:?}\n", temp.convert_to_ref_scale());
    
    println!("Result of conversion to {:?} is: {:?}", Scale::Celsius, temp.convert_to(Scale::Celsius));
    println!("Result of conversion to {:?} is: {:?}", Scale::Fahrenheit, temp.convert_to(Scale::Fahrenheit));
    println!("Result of conversion to {:?} is: {:?}", Scale::Romer, temp.convert_to(Scale::Romer));
}