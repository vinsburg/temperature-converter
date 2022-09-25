// Data source: [Conversion of scales of temperature](https://en.wikipedia.org/wiki/Conversion_of_scales_of_temperature)
const ORIG_SCALE: fn() -> Scale = kelv;

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
        ratio: 21.0/40.0,
        del_x: -273.15,
        del_y: 7.5,
    }
}

fn dest_to_orig(scale: Scale, dest_val: f64) -> f64 {
    (dest_val + scale.del_x) * scale.ratio + scale.del_y
}

fn orig_to_dest(scale: Scale, orig_val: f64) -> f64 {
    (orig_val - scale.del_y) / scale.ratio - scale.del_x
}

fn dest_to_dest(scale_a: Scale, scale_b: Scale, dest_a_val: f64) -> f64 {
    orig_to_dest(
        scale_b,
        dest_to_orig(
            scale_a,
            dest_a_val,
        ),
    )
}

fn print_dest_to_dest(scale_a: fn() -> Scale, scale_b: fn() -> Scale, dest_a_val: f64) {
    let dest_b_val = dest_to_dest(scale_a(), scale_b(), dest_a_val);
    println!("{dest_a_val:.2} {:10} is {dest_b_val:.2} {:10}", scale_a().name, scale_b().name);
}

fn print_dest_to_orig(scale: fn() -> Scale, dest_val: f64) {
    let orig_val = dest_to_orig(scale(), dest_val);
    println!("{dest_val:.2} {:10} is {orig_val:.2} {:10}", scale().name, ORIG_SCALE().name);
}

fn main() {
    let temperature = 0.0;

    print_dest_to_orig(cels, temperature);
    print_dest_to_orig(fahr, temperature);
    print_dest_to_orig(rome, temperature);

    print_dest_to_dest(cels, fahr, temperature);
    print_dest_to_dest(fahr, cels, temperature);
    print_dest_to_dest(cels, rome, temperature);
}