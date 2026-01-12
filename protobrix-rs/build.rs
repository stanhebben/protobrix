use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    // Enable serde serialization for all types
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    // Compile the proto file
    config.compile_protos(&["../protobuf/main_element.proto"], &["../protobuf/"])?;

    Ok(())
}
