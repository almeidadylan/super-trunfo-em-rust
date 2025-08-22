use std::io;

pub fn read_int_data () -> usize {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read data");
    data.trim().parse().expect("Error converting data to integer")
}