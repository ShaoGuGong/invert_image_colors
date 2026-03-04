use cc;

fn main() {
    println!("cargo:rerun-if-changed=src/invert.cu");

    let mut builder = cc::Build::new();
    let cuda_path = "/usr/local/cuda";

    builder
        .cuda(true)
        .file("src/invert.cu")
        .flag("-arch=sm_100")
        .flag("-O3");
    println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
    println!("cargo:rustc-link-lib=cudart");
    builder.compile("cuda_kernel");
}
