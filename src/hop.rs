use ockam::{Any, Context, Result, Routed, Worker};

pub struct Hop;

#[ockam::worker]
impl Worker for Hop {
    type Context = Context;
    type Message = Any;

    /// This handle function tales any incoming message and forwards
    /// it to the next hop in it's onward route
    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<Any>) -> Result<()> {
        println!("Address: {}, Received: {}", ctx.address(), msg);

        // some type conversion
        let mut message = msg.into_local_message();
        let transport_message = message.transport_mut();

        // remove my address from the onward_route
        transport_message.onward_route.step()?;

        // insert my address at the begining return_route
        transport_message
            .return_route
            .modify()
            .prepend(ctx.address());

        // send the message on its onward_route
        ctx.forward(message).await
    }
}
