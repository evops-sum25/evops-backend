use std::env;
use std::path::PathBuf;

fn main() -> eyre::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("api-descriptor.bin"))
        .compile_protos(
            &["../../client-ext/proto/evops/api/v1/api.proto"],
            &["../../client-ext/proto/"],
        )?;

    Ok(())
}
