
extern crate cc;
extern crate bindgen;


pub fn main() {


/*
	// Compile cpp files into static library
	cc::Build::new()
        .cpp(true)
		.file("uWebSockets/src/App.h")
        .include("uWebSockets/uSockets/src")
		//.file("libfoo/foo.cpp")  
        .flag("-std=gnu++17")   
		.compile("App");
*/

    let bindings = bindgen::Builder::default()
        .header("uWebSockets/src/App.h")
        .clang_arg("-I./uWebSockets/uSockets/src")   
        .clang_arg("-std=c++17")
        .clang_arg("-x")
        .clang_arg("c++")
     //   .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");


}
