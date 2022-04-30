use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../../third_party/proto/cosmos/staking/v1beta1/query.proto"],
        &["../../third_party/proto/"], // NOTE: must have the slash in the end, i.e. `proto/` not `proto`
    )?;
    Ok(())
}
