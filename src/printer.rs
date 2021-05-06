use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn write_to_file(best : i32, time : f64, filename : std::string::String) {
    let file = File::create(&filename).unwrap();
    let mut writer = BufWriter::new(&file);
    write!(&mut writer, "{}\t{}", best, time);
}