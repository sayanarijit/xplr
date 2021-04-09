use crate::app::{DirectoryBuffer, ExplorerConfig, ExternalMsg, InternalMsg, MsgIn, Node, Task};
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
        fs::read_dir(&path)
            .map(|dirs| {
                dirs.filter_map(|d| {
                    d.ok().map(|e| {
                        e.path()
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default()
                    })
                })
                .map(|name| Node::new(parent.clone(), name))
                .filter(|n| config.filter(n))
                .collect::<Vec<Node>>()
            })
            .map(|nodes| {
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
                    MsgIn::Internal(InternalMsg::AddDirectory(parent, dir)),
                    None,
                ))
                .unwrap();
            })
            .unwrap_or_else(|e| {
                tx.send(Task::new(
                    MsgIn::External(ExternalMsg::LogError(e.to_string())),
                    None,
                ))
                .unwrap();
            })
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
