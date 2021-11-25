// this node starts a tcp listener and an echoer worker
// it then runs forever waiting for messages

use hello_ockam::Echoer;
use ockam::{Context, Result, TcpTransport};

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // initialize the TCP Transport
    let tcp = TcpTransport::create(&ctx).await?;

    // create a TCP listener and wait for incoming connections
    tcp.listen("127.0.0.1:4000").await?;

    // create an echoer worker
    ctx.start_worker("echoer", Echoer).await?;

    // don't call ctx.stop() here so this ndoe runs forever
    Ok(())
}
