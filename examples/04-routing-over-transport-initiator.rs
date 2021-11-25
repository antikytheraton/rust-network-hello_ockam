// this node routes a message, to a worker on a different node, over tha tcp transport

use ockam::{route, Context, Result, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // initialize the TCP Transport
    let _tcp = TcpTransport::create(&ctx).await?;

    // send a message to the "echoer" worker, on a different node, over a tcp transport
    let r = route![(TCP, "localhost:4000"), "echoer"];
    ctx.send(r, String::from("Hello Ockam!")).await?;

    // wait th receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // stop all workers, stop the node, clenup and return
    ctx.stop().await
}
