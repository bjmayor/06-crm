use std::{fs, process::Command};

use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
fn main() -> Result<()> {
    // 编译到了 OUT_DIR 目录下
    // prost_build::compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    // 自定义输出目录
    fs::create_dir_all("./src/pb/")?;
    // let mut config = prost_build::Config::new();
    // config
    //     .out_dir("./src/pb/")
    //     .compile_protos(&["../protos/crm.proto"], &["../protos"])?;
    // let builder = tonic_build::configure();
    // builder.out_dir("./src/pb/").compile(
    //     &[
    //         "../protos/user-stats/messages.proto",
    //         "../protos/user-stats/rpc.proto",
    //     ],
    //     &["../protos"],
    // )?;

    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_serde(
            &["User"],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_sqlx_from_row(&["User"], None)
        .with_derive_builder(
            &[
                "User",
                "QueryRequest",
                "RawQueryRequest",
                "TimeQuery",
                "IdQuery",
            ],
            None,
        )
        .with_field_attributes(
            &["User.email", "User.name", "RawQueryRequest.query"],
            &[r#"#[builder(setter(into))]"#],
        )
        .with_field_attributes(
            &["TimeQuery.before", "TimeQuery.after"],
            &[r#"#[builder(setter(into, strip_option))]"#],
        )
        .with_field_attributes(
            &["QueryRequest.timestamps"],
            &[r#"#[builder(setter(each(name="timestamp", into)))]"#],
        )
        .with_field_attributes(
            &["QueryRequest.ids"],
            &[r#"#[builder(setter(each(name="id", into)))]"#],
        )
        .compile(
            &[
                "../protos/user-stats/messages.proto",
                "../protos/user-stats/rpc.proto",
            ],
            &["../protos"],
        )
        .unwrap();
    Command::new("cargo").args(["fmt"]).output().unwrap();
    Ok(())
}
