use std::path::PathBuf;

fn main() {
    #[cfg(feature = "desktop")]
    tonic_build::configure()
        .out_dir(PathBuf::from("src/desktop"))
        .compile_protos(&["proto/stream.proto"], &["proto"])
        .unwrap();

    #[cfg(feature = "web")]
    prost_build::Config::new()
        .out_dir(PathBuf::from("src/web"))
        .compile_protos(&["proto/stream.proto"], &["proto"])
        .unwrap();
}
