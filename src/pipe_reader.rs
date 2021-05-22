use crate::app::ExternalMsg;
use anyhow::Result;
use std::fs;
use std::io::prelude::*;

pub fn read_all(pipe: &str) -> Result<Vec<ExternalMsg>> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(&pipe)?;

    let mut in_str = String::new();
    file.read_to_string(&mut in_str)?;
    file.set_len(0)?;

    if !in_str.is_empty() {
        let mut msgs = vec![];
        for msg in in_str.lines().map(|s| serde_yaml::from_str(s.trim())) {
            msgs.push(msg?);
        }
        Ok(msgs)
    } else {
        Ok(vec![])
    }
}

// pub fn keep_reading(pipe: String, tx: Sender<Task>) {
//     let mut last_modified = None;
//     thread::spawn(move || loop {
//         let path = PathBuf::from(&pipe);

//         if !path.exists() {
//             thread::sleep(Duration::from_millis(50));
//             continue;
//         }

//         let modified = path.metadata().and_then(|m| m.modified()).ok();

//         if modified == last_modified {
//             thread::sleep(Duration::from_millis(50));
//         } else if let Ok(mut file) = fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(false)
//             .open(&pipe)
//         {
//             let mut in_str = String::new();
//             file.read_to_string(&mut in_str).unwrap_or_default();
//             file.set_len(0).unwrap_or_default();

//             if !in_str.is_empty() {
//                 let msgs = in_str
//                     .lines()
//                     .map(|s| serde_yaml::from_str::<ExternalMsg>(s.trim()));

//                 msgs.for_each(|msg| match msg {
//                     Ok(m) => {
//                         tx.send(Task::new(MsgIn::External(m), None))
//                             .unwrap_or_default();
//                     }
//                     Err(e) => {
//                         tx.send(Task::new(
//                             MsgIn::External(ExternalMsg::LogError(e.to_string())),
//                             None,
//                         ))
//                         .unwrap_or_default();
//                     }
//                 });
//             };
//         } else {
//             tx.send(Task::new(
//                 MsgIn::External(ExternalMsg::LogError(format!(
//                     "Failed to open input pipe: {}",
//                     &pipe
//                 ))),
//                 None,
//             ))
//             .unwrap_or_default();
//             thread::sleep(Duration::from_secs(3));
//         }

//         last_modified = modified;
//     });
// }
