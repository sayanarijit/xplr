use crate::app::DirectoryBuffer;
use crate::app::ExplorerConfig;
use crate::app::Node;
use crate::app::Task;
use crate::app::{InternalMsg, MsgIn};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;

pub fn explore(
    config: ExplorerConfig,
    parent: String,
    focused_path: Option<String>,
    tx: Sender<Task>,
) {
    let path = PathBuf::from(&parent);
    let path_cloned = path.clone();
    let tx_cloned = tx.clone();
    let config_cloned = config.clone();

    thread::spawn(move || {
        let nodes: Vec<Node> = fs::read_dir(&path)
            .unwrap()
            .filter_map(|d| {
                d.ok().map(|e| {
                    e.path()
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default()
                })
            })
            .map(|name| Node::new(parent.clone(), name))
            .filter(|n| config.apply(n))
            .collect();

        let focus_index = if let Some(focus) = focused_path {
            nodes
                .iter()
                .enumerate()
                .find(|(_, n)| n.relative_path == focus)
                .map(|(i, _)| i)
                .unwrap_or(0)
        } else {
            0
        };

        let dir = DirectoryBuffer::new(parent.clone(), nodes, focus_index);

        tx.send(Task::new(
            1,
            MsgIn::Internal(InternalMsg::AddDirectory(parent, dir)),
            None,
        ))
        .unwrap();
    });

    if let Some(grand_parent) = path_cloned.parent() {
        explore(
            config_cloned,
            grand_parent.to_string_lossy().to_string(),
            path_cloned
                .file_name()
                .map(|f| f.to_string_lossy().to_string()),
            tx_cloned,
        );
    }
}
