pub fn load_input(year: u16, day: u8) -> String {
    let profile = std::env::var("AOC_PROFILE").unwrap_or("default".to_string());
    let filename = format!("{}-{}-{:02}-input.txt", profile, year, day);
    let filepath = std::path::PathBuf::from(format!("../utils/.cache/{}", filename));
    std::fs::read_to_string(filepath.clone()).unwrap_or_else(|_| {
        panic!(
            "{}",
            format!("Could not read input file: {:?}", filepath).to_string()
        )
    })
}
