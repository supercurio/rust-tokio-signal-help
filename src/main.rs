extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;
extern crate tokio_io;

use futures::stream::Stream;
use tokio_core::reactor::Core;

use futures::Future;
use tokio_core::reactor::Handle;
use tokio_io::{IoStream, IoFuture};
use tokio_signal::unix;


fn sigint_imp(handle: &Handle) -> IoFuture<IoStream<()>> {
    unix::Signal::new(unix::libc::SIGINT, handle)
        .map(|x| x.map(|_| ()).boxed())
        .boxed()
}
fn sigterm_imp(handle: &Handle) -> IoFuture<IoStream<()>> {
    unix::Signal::new(unix::libc::SIGTERM, handle)
        .map(|x| x.map(|_| ()).boxed())
        .boxed()
}

fn main() {
    let mut core = Core::new().unwrap();
    let sigint = sigint_imp(&core.handle());
    let sigterm = sigint_imp(&core.handle());
    
    let stream = core.run(sigint).unwrap();

    core.run(stream.for_each(|()| {
        println!("Ctrl-C received!");
        Ok(())
    })).unwrap();

}