// this node creates an end-to-end encrypted secure channel over two tcp transport hops
// it then routes a message, to a worker on a different node, through this encrypted channel

use ockam::{route, Context, Result, TrustEveryonePolicy, Vault};
use ockam::{Entity, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // initialize the TCP Transport
    TcpTransport::create(&ctx).await?;

    // create a vault to safely store secret keys for Alice
    let vault = Vault::create(&ctx).await?;

    let mut alice = Entity::create(&ctx, &vault).await?;

    // connect to a secure channel listener and perform a handshake
    let r = route![
        (TCP, "localhost:3000"),
        (TCP, "localhost:4000"),
        "bob_listener"
    ];
    let channel = alice.create_secure_channel(r, TrustEveryonePolicy).await?;

    // send a message to the echoer worker via the channel
    ctx.send(route![channel, "echoer"], String::from("Hello Ockam!"))
        .await?;

    // wait to receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // stop all workers, stop the node, cleanup and return
    ctx.stop().await
}
