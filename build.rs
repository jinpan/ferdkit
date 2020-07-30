extern crate cc;

fn main() {
    // Note: setup.sh should be run to download rdkit.

    let dst = cmake::Config::new("rdkit")
        .generator("Unix Makefiles")
        .define("RDK_BUILD_PYTHON_WRAPPERS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=RDKitSmilesParse");

    cc::Build::new()
        .file("src/wrapper.cc")
        .include("./rdkit/Code")
        .cpp(true)
        .compile("libwrapper.a");
}