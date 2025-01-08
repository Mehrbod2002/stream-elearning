use std::path::PathBuf;

fn main() {
    tonic_build::configure()
        .out_dir(PathBuf::from("src"))
        .compile_protos(&["proto/stream.proto"], &["proto"])
        .unwrap();
}
