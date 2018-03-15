fn main() {
    println!("cargo:rustc-link-search=/Users/ohadr/anaconda/envs/testpypyclean/lib");
    println!("cargo:rustc-link-search=/Users/ohadr/anaconda/envs/testpypyclean/include");
    println!("cargo:include=/Users/ohadr/anaconda/envs/testpypyclean/include");
    //println!("cargo:rustc-flags=-C link-args='-Wl,-undefined,dynamic_lookup'");
    println!("cargo:rustc-link-lib=pypy3-c");

}
