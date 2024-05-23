fn main() {
    cc::Build::new().file("src/sockets.c").compile("sockets");
}
