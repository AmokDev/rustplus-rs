fn main() {
    prost_build::compile_protos(&["proto/rustplus.proto"], &["proto/"]).unwrap();
}
