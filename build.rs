use std::io::Result;

const PROTO_FILE: &str = "protos/google/protobuf/types.proto";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={}", PROTO_FILE);

    //let types = pbjson_utils::parse_proto_file(PROTO_FILE).expect("Unable to parse proto file!");

    //println!("types:{types:#?}");

    let mut builder = tonic_build::configure()
        //.out_dir("src/protobuf")
        .build_client(false)
        .build_server(false)
        .compile_well_known_types(true)
        .disable_comments(".")
        .bytes([".google"]);

    builder = builder.type_attribute("Shirt", "#[derive( serde::Serialize, serde::Deserialize)]");

    builder = builder.type_attribute(
        "UInt32Value",
        "#[derive( serde::Serialize, serde::Deserialize)]",
    );

    builder = builder.type_attribute(
        "StringValue",
        "#[derive( serde::Serialize, serde::Deserialize)]",
    );

    builder = builder.type_attribute("FloatValue", "#[derive(pbjson_derive::SerializeFloat32)]");

    // let mut config = prost_build::Config::new();
    // config
    //     //.file_descriptor_set_path(&descriptor_path)
    //     .compile_well_known_types()
    //     .disable_comments(["."])
    //     .bytes([".google"])
    //     //.skip_protoc_run()
    //     ;

    //let empty: &[&str] = &[];
    //config.compile_protos(empty, empty)?;

    //config.compile_protos(&[PROTO_FILE], &["protos/google/protobuf/"])?;

    builder.compile(&[PROTO_FILE], &["protos/google/protobuf/"])?;
    Ok(())
}
