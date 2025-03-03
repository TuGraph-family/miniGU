use std::io::Result;

fn main() -> Result<()> {
    let mut prost_config = prost_build::Config::new();
    
    let proto_file = "src/wal.proto";
    
    prost_config.compile_protos(&[proto_file], &["src/"])?;
    
    Ok(())
}