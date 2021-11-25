// this node routes a message

use hello_ockam::{Echoer, Hop};
use ockam::{route, Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    //start a worker, of type Echoer, at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // start a worker, of type Hop, at address "h1"
    ctx.start_worker("h1", Hop).await?;

    // send a message to the worker at address "echoer",
    // via the worker at address "h1"
    ctx.send(route!["h1", "echoer"], String::from("Hello Ockam!"))
        .await?;
    // wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // stop all workers, stop the node, clenup and return.
    ctx.stop().await
}
