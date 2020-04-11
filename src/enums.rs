// Enums are types that have a few definite values

// Define enum
enum Enum {
    One,
    Two,
    Three,
    Four,
}

fn get_enum_value(e: Enum) {
    match e {
        Enum::One => println!("Get enum value: One"),
        Enum::Two => println!("Get enum value: Two"),
        Enum::Three => println!("Get enum value: Three"),
        Enum::Four => println!("Get enum value: Four"),
    }
}

pub fn main() {
    // Define enum
    let enum1 = Enum::One;
    let enum2 = Enum::Two;
    let enum3 = Enum::Three;
    let enum4 = Enum::Four;

    get_enum_value(enum1);
    get_enum_value(enum2);
    get_enum_value(enum3);
    get_enum_value(enum4);
}
