fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/data_node.proto")?;
    tonic_build::compile_protos("../../proto/main_server.proto")?;
    tonic_build::compile_protos("../../proto/registry_main_server.proto")?;
    Ok(())
}
