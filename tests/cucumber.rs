use cucumber_rust::{async_trait, criteria::feature, futures::FutureExt, Cucumber, World};
use std::convert::Infallible;
use std::fs;
use xplr::app;

mod steps;

pub enum MyWorld {
    Nothing,
    App(app::App),
    MsgOutList(Vec<app::MsgOut>),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<MyWorld>::new()
        // Specifies where our feature files exist
        .features(&["./features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::key_bindings::steps())
        // Add some lifecycle functions to manage our database nightmare
        .before(feature("Example feature"), |_| {
            async move {
                fs::create_dir_all("/tmp/xplr_bdd").unwrap();
                (1..100).for_each(|i| {
                    fs::File::create(format!("/tmp/xplr_bdd/{}", i)).unwrap();
                })
            }
            .boxed()
        })
        .after(feature("Example feature"), |_| {
            async move {
                fs::create_dir_all("/tmp/xplr_bench").unwrap();
                (1..100).for_each(|i| {
                    fs::remove_file(format!("/tmp/xplr_bdd/{}", i)).unwrap();
                })
            }
            .boxed()
        })
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
