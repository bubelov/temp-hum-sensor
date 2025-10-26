fn main() {
    // hey linker, use the linkall.x that came with esp-rt
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
