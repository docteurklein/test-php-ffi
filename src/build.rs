extern crate cbindgen;

fn main() {
    cbindgen::Builder::new()
      .with_crate(".")
      .with_config(cbindgen::Config::from_file("./cbindgen.toml").unwrap())
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}
