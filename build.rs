use std::io::Result;

const PROTO_FILE: &str = "protos/google/protobuf/types.proto";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={}", PROTO_FILE);

    let mut builder = tonic_build::configure()
        //.out_dir("src/protobuf")
        .build_client(false)
        .build_server(false)
        .compile_well_known_types(true)
        .disable_comments(".")
        .bytes([".google"]);

    // type names can be parsed out and type attribute can be applied accross the board, default
    // while custom ser/deser can be applied on per-case...
    //
    //

    //make it serializable (serde_json or any other )

    builder = builder.type_attribute("Shirt", "#[derive( serde::Serialize, serde::Deserialize)]");

    //make Well Known serializable (serde_json or any other )
    builder = builder.type_attribute(
        "UInt32Value",
        "#[derive( serde::Serialize, serde::Deserialize)]",
    );

    //make Well Known serializable (serde_json or any other )
    builder = builder.type_attribute(
        "StringValue",
        "#[derive( serde::Serialize, serde::Deserialize)]",
    );

    //Custom serializer
    builder = builder.type_attribute("FloatValue", "#[derive(pbjson_derive::SerializeFloat32)]");

    builder.compile(&[PROTO_FILE], &["protos/google/protobuf/"])?;
    Ok(())
}
