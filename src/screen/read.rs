use std::io;

pub fn read_int_data () -> usize {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read data");
    data.trim().parse().expect("Error converting data to integer")
}

pub fn read_float_data () -> f64 {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Railed to read data");
    let value: f64 = data.trim().parse().expect("Error converting data to float");
    let value_format = format!("{:.2}", value).parse().unwrap();
    value_format
}

pub fn read_string_data () -> String {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read data");
    data.trim().to_string()
}