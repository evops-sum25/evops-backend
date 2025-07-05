fn main() -> eyre::Result<()> {
    tonic_build::configure().compile_protos(
        &["../../server-ext/proto/evops/ml/v1/ml.proto"],
        &["../../server-ext/proto/"],
    )?;
    Ok(())
}
