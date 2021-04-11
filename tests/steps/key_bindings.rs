use cucumber_rust::Steps;

use crate::MyWorld;
use xplr::app;
use xplr::input::Key;

pub fn steps() -> Steps<MyWorld> {
    let mut steps: Steps<MyWorld> = Steps::new();

    steps.given("xplr is running", |_world, _ctx| {
        let app = app::App::create("/tmp/xplr_bdd".into()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
        MyWorld::App(app)
    });

    steps.when_regex(r#"I press "(.*)"$"#, |world, ctx| match world {
        MyWorld::App(app) => {
            let k = &ctx.matches[1];
            let key: Key = serde_yaml::from_str(&k).unwrap_or_else(|_| k.clone().into());
            let msg = app::MsgIn::Internal(app::InternalMsg::HandleKey(key.clone()));
            let task = app::Task::new(msg, Some(key));
            MyWorld::App(app.handle_task(task).unwrap())
        }
        _ => {
            panic!("invalid world state")
        }
    });

    steps.when("xplr processes the tasks", |world, _ctx| match world {
        MyWorld::App(app) => {
            let mut app = app;
            let mut msgs = vec![];
            while let Some(msg) = app.pop_msg_out() {
                match msg {
                    app::MsgOut::Enque(task) => {
                        app = app.handle_task(task).unwrap();
                    }
                    msg => {
                        msgs.push(msg);
                    }
                }
            }
            MyWorld::MsgOutList(msgs)
        }
        _ => {
            panic!("invalid world state")
        }
    });

    steps.then_regex(r#"xplr performs "(.*)"$"#, |world, ctx| {
        let mstr = &ctx.matches[1];
        let msg = serde_yaml::from_str(&mstr).unwrap();
        match world {
            MyWorld::MsgOutList(msgs) => {
                let mut msgs = msgs.into_iter();
                assert_eq!(msgs.next(), Some(msg));
                MyWorld::MsgOutList(msgs.collect())
            }

            _ => {
                panic!("invalid world state")
            }
        }
    });

    steps.then(
        "xplr quits or waits for the next event",
        |world, _ctx| match world {
            MyWorld::MsgOutList(msgs) => {
                let mut msgs = msgs.into_iter();
                assert_eq!(msgs.next(), None);
                MyWorld::Nothing
            }

            _ => {
                panic!("invalid world state")
            }
        },
    );

    steps
}
