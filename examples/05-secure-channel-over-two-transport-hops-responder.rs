// this node starts a tcp listener, a secure channel listener, and a echoer worker
// it then runs forever waiting for messages

use hello_ockam::Echoer;
use ockam::{Context, Entity, Result, TcpTransport, TrustEveryonePolicy, Vault};

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    ctx.start_worker("echoer", Echoer).await?;

    // initialize the TCP Transport
    let tcp = TcpTransport::create(&ctx).await?;

    // create a TCP listener and wait for incoming connections
    tcp.listen("127.0.0.1:4000").await?;

    // create a vault to safely store secret keys for Bob
    let vault = Vault::create(&ctx).await?;

    // create an entity to represent Bob
    let mut bob = Entity::create(&ctx, &vault).await?;

    bob.create_secure_channel_listener("bob_listener", TrustEveryonePolicy)
        .await?;

    // don't call ctx.stop() here so this node runs forever
    Ok(())
}
