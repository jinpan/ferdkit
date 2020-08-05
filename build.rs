fn main() {
    // The wrapper should first be built via `make wrapper` in the wrapper dir.

    println!("cargo:rustc-link-search=wrapper");
    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rustc-link-search=rdkit/lib");
    println!("cargo:rustc-link-lib=static=RDKitSmilesParse_static");

    println!("cargo:rustc-link-lib=stdc++");
}