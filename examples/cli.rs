use theme_color_extractor::extract;

fn main() {
    let files = std::env::args().skip(1);
    if files.len() == 0 {
        eprintln!("Usage: {} <image>...", std::env::args().next().unwrap());
        std::process::exit(1);
    }
    for file in files {
        let image = std::fs::read(&file).unwrap();
        let file_name = file.split('/').last().unwrap();
        let result = extract(image);
        println!(
            "{file:>32}: #{r:02x}{g:02x}{b:02x} \x1b[48;2;{r};{g};{b}m    \x1b[0m",
            file = file_name,
            r = result.r,
            g = result.g,
            b = result.b
        );
    }
}
