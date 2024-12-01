pub fn load_input(year: u16, day: u8) -> String {
    let filename = format!("{}-{:02}-input.txt", year, day);
    let filepath = std::path::PathBuf::from(format!("../utils/.cache/{}", filename));
    std::fs::read_to_string(filepath).expect("Could not read input file.")
}
