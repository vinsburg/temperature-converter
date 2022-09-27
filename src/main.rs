// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
#[derive(Debug)]
struct Scale {
    name: String,
    ratio: f64,
    del_x: f64,
    del_y: f64,
}

fn kelv() -> Scale {
    Scale {
        name: String::from("Kelvin"),
        ratio: 1.0,
        del_x: 0.0,
        del_y: 0.0,
    }
}

fn fahr() -> Scale {
    Scale {
        name: String::from("Fahrenheit"),
        ratio: 5.0/9.0,
        del_x: 459.67,
        del_y: 0.0,
    }
}

fn cels() -> Scale {
    Scale {
        name: String::from("Celsius"),
        ratio: 1.0,
        del_x: 273.15,
        del_y: 0.0,
    }
}

fn rome() -> Scale {
    Scale {
        name: String::from("Rømer"),
        ratio: 40.0/21.0,
        del_x: -7.5,
        del_y: 273.15,
    }
}

fn dest_to_orig(scale: Scale, dest_val: f64) -> f64 {
    (dest_val + scale.del_x) * scale.ratio + scale.del_y
}

fn orig_to_dest(scale: Scale, orig_val: f64) -> f64 {
    (orig_val - scale.del_y) / scale.ratio - scale.del_x
}

fn convert(scale_a: Scale, scale_b: Scale, val_a: f64) -> f64 {
    orig_to_dest(
        scale_b,
        dest_to_orig(
            scale_a,
            val_a,
        ),
    )
}

fn display_conversion(scale_a: fn() -> Scale, scale_b: fn() -> Scale, val_a: f64) {
    let dest_b_val = convert(scale_a(), scale_b(), val_a);
    println!("{val_a:.2} {:10} is {dest_b_val:.2} {:10}", scale_a().name, scale_b().name);
}

fn main() {
    println!("{:?}", kelv());
    println!("{:?}", cels());
    println!("{:?}", fahr());
    println!("{:?}", rome());

    let temperature = 0.0;

    display_conversion(cels, fahr, temperature);
    display_conversion(fahr, cels, temperature);
    display_conversion(cels, rome, temperature);
}