// Structs are used to create a custom data type

// Traditional struct
struct TraditionalStruct {
    string: String,
    integer: i32,
}

// Tuple struct
struct TupleStruct(String, i32);

// Function struct
struct FunctionStruct {
    string: String,
    integer: i32,
}

impl FunctionStruct {
    fn new(string: &str, integer: i32) -> FunctionStruct {
        FunctionStruct {
            string: string.to_string(),
            integer: integer,
        }
    }

    fn get_data(&self) -> String {
        format!("{} {}", self.string, self.integer)
    }

    fn set_string(&mut self, string: &str) {
        self.string = string.to_string();
    }

    fn set_integer(&mut self, integer: i32) {
        self.integer = integer;
    }
}

pub fn main() {
    // Traditional struct
    let mut traditional_struct = TraditionalStruct {
        string: String::from("TraditionalStruct"),
        integer: 50,
    };
    println!(
        "Traditional struct: {} {}",
        traditional_struct.string, traditional_struct.integer
    );

    // Update struct property
    traditional_struct.string = String::from("Traditional Struct");
    traditional_struct.integer = 100;
    println!(
        "Update struct property: {} {}",
        traditional_struct.string, traditional_struct.integer
    );

    // Tuple struct
    let tuple_struct = TupleStruct(String::from("TupleStruct"), 50);
    println!("Tuple struct: {} {}", tuple_struct.0, tuple_struct.1);

    // Function struct
    let mut function_struct = FunctionStruct::new("FunctionStruct", 50);
    println!("Function struct: {}", function_struct.get_data());

    // Update function struct
    function_struct.set_string("Function Struct");
    function_struct.set_integer(100);
    println!("Update function struct: {}", function_struct.get_data());
}
