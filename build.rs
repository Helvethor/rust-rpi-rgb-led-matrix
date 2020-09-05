extern crate gcc;

fn main() {
    #[cfg(not(feature = "rgbmatrix-mock"))]
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
