fn main() {
    println!("cargo:rustc-link-search=/Users/ohadr/anaconda/envs/windmillpypy/lib");
    println!("cargo:rustc-link-lib=pypy3-c");
}
