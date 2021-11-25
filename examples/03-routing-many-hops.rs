// this node routes a message throug many hops

use hello_ockam::{Echoer, Hop};
use ockam::{route, Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    //start an echoer worker at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // start 3 hop workers at addresses "h1", "h2" and "h3"
    ctx.start_worker("h1", Hop).await?;
    ctx.start_worker("h2", Hop).await?;
    ctx.start_worker("h3", Hop).await?;

    // send a message to the echoer worker via the "h1", "h2", and "h3" workers
    let r = route!["h1", "h2", "h3", "echoer"];
    ctx.send(r, String::from("Hello Ockam!")).await?;

    // wait to receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    //stop all workers, stop the node, clenup and return
    ctx.stop().await
}
