extern crate fantoccini;
extern crate futures;
extern crate tokio_core;

pub fn greetings() -> String {
    println!("Hello from rad_engine!");
    String::from("Hello from rad_engine!")
}

pub fn run() -> String {
    use fantoccini::{Client, Locator};
    use futures::future::Future;
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let c = Client::new("http://localhost:4444", &core.handle());
    let c = core.run(c).unwrap();

    {
        // we want to have a reference to c so we can use it in the and_thens below
        let c = &c;

        // now let's set up the sequence of steps we want the browser to take
        // first, go to the Wikipedia page for Foobar
        let f = c.goto("https://en.wikipedia.org/wiki/Foobar")
            .and_then(move |_| c.current_url())
            .and_then(move |url| {
                assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");
                // click "Foo (disambiguation)"
                c.find(Locator::Css(".mw-disambig"))
            })
            .and_then(|e| e.click())
            .and_then(move |_| {
                // click "Foo Lake"
                c.find(Locator::LinkText("Foo Lake"))
            })
            .and_then(|e| e.click())
            .and_then(move |_| c.current_url())
            .and_then(|url| {
                assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");
                Ok(())
            });

        // and set the browser off to do those things
        core.run(f).unwrap();
    }

    // drop the client to delete the browser session
    if let Some(fin) = c.close() {
        // and wait for cleanup to finish
        core.run(fin).unwrap();
    }
    String::from("hello")
}