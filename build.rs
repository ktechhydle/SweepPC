fn main() {
    println!("cargo:rerun-if-changed=resources.res");
    println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE"); // this is a console app!
    println!("cargo:rustc-link-arg=resources.res");
}
