use std::fs;

use anyhow::Result;
fn main() -> Result<()> {
    // 编译到了 OUT_DIR 目录下
    // prost_build::compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    // 自定义输出目录
    fs::create_dir_all("./src/pb/")?;
    // let mut config = prost_build::Config::new();
    // config
    //     .out_dir("./src/pb/")
    //     .compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    let builder = tonic_build::configure();
    builder
        .out_dir("./src/pb/")
        .compile(&["../protos/crm.proto"], &["../protos"])?;
    Ok(())
}
