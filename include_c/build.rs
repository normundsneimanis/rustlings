fn main() {
  println!("cargo:rustc-link-search={}", "/home/lanctot/rust_test");
  println!("cargo:rustc-link-lib=dylib=test");
}
