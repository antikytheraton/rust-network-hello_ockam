use ockam::{Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    ctx.stop().await
}
