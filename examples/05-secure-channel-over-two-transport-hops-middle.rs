// starts a tcp listener at 127.0.0.1:3000
// it then runs forever waiting to route messages

use ockam::{Context, Result, TcpTransport};

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // initialize the TCP Transport
    let tcp = TcpTransport::create(&ctx).await?;

    // create a TCP listener and wait for incoming connections
    tcp.listen("127.0.0.1:3000").await?;

    // don't call ctx.stop() here so this node runs forever
    Ok(())
}
