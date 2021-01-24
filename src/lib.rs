pub trait Compile {
    fn from_src(src: &str) {
        println!("Compiling from source: {}", src);
    }
}
