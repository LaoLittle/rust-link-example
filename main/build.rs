fn main() {
    println!("cargo:rustc-link-search=native=target/debug");
    // $ORIGIN: origin path of bin
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    println!("cargo:rustc-link-lib=dylib=lib");
}