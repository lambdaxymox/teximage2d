extern crate texture;
extern crate texture_gen;

use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Write;


fn generate_code_fragment<P: AsRef<Path>>(path: P) -> String {
    let model = texture::load_file(path).unwrap();
    let fragment = texture_gen::to_rust_code(&model.image);

    fragment
}

fn write_code_fragment(fragment: &str, fragment_name: &str) -> io::Result<()> {
    let path = Path::new("tests").join(fragment_name);
    let mut file = File::create(&path)?;
    file.write_all(fragment.as_bytes())?;
    file.sync_all()
}

fn main() -> io::Result<()> {
    let fragment = generate_code_fragment("assets/sample.png");
    write_code_fragment(&fragment, "sample_png_test.in")
}
