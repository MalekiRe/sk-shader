use std::env;
use std::fs::read_dir;
use std::path::PathBuf;
use std::str::FromStr;

macro_rules! cargo_link {
	($feature:expr) => {
		println!("cargo:rustc-link-lib={}", $feature);
	};
}

fn main() {
    println!("cargo:rustc-env=PROC_ARTIFACT_DIR={}",
             std::env::var("OUT_DIR").unwrap() );

    let mut cmake_config = cmake::Config::new("skshaderc");
    let dst = cmake_config.build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    let mut path = PathBuf::from_str(&dst.display().to_string()).unwrap();
    path.push("lib");
    for entry in read_dir(path).unwrap() {
        if let Ok(entry) = entry {
            if !entry.file_type().unwrap().is_file() {
                continue;
            }
            let mut name = entry.file_name().to_str().unwrap().to_string();
            if !name.ends_with(".a") {
                continue;
            }
            if name.contains("libSPVRemapper.a") {
                continue;
            }
            name.pop();
            name.pop();
            name.remove(0);
            name.remove(0);
            name.remove(0);
            println!("cargo:rustc-link-lib=static={}", name);
        }
    }
    /*cargo_link!("static=spirv-cross-c");
    cargo_link!("static=spirv-cross-core");
    cargo_link!("static=spirv-cross-cpp");
    cargo_link!("static=spirv-cross-hlsl");
    cargo_link!("static=spirv-cross-reflect");
    cargo_link!("static=spirv-cross-util");
    cargo_link!("static=SPIRV-Tools");
    cargo_link!("static=SPIRV-Tools-diff");
    cargo_link!("static=SPIRV-Tools-link");
    cargo_link!("static=SPIRV-Tools-lint");
    cargo_link!("static=SPIRV-Tools-opt");
    cargo_link!("static=SPIRV-Tools-reduce");
    cargo_link!("static=glslang");*/
    //cargo_link!("static=SPIRV-Tools-shared");
    //println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=skshaderc");
    cargo_link!("stdc++");
    //println!("cargo:rustc-link-search=native={}/lib", dst.display());


    println!("cargo:rerun-if-changed=src/static-wrapper.h");
    //
    // let bindings = bindgen::Builder::default()
    //     .header("src/static-wrapper.h")
    //     .generate()
    //     .expect("unable to generate bindings");

    /*let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/
}