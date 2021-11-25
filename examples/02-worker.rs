use hello_ockam::Echoer;
use ockam::{Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // start a worker, of type Echoer, at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // send a message to the worker at address "echoer"
    ctx.send("echoer", String::from("Hello Ockam!")).await?;

    // wait to receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App Received: {}", reply);

    // stop all workers, stop the node, cleanup and return
    ctx.stop().await
}
