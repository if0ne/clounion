fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/data_node_api.proto")?;
    tonic_build::compile_protos("../../proto/main_server_api.proto")?;
    Ok(())
}
