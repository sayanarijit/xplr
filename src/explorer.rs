use crate::app::{DirectoryBuffer, ExplorerConfig, ExternalMsg, InternalMsg, MsgIn, Node, Task};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;

pub fn explore_sync(
    config: ExplorerConfig,
    parent: String,
    focused_path: Option<String>,
) -> Result<DirectoryBuffer> {
    let path = PathBuf::from(&parent);

    let dirs = fs::read_dir(&path)?;
    let mut nodes = dirs
        .filter_map(|d| {
            d.ok().map(|e| {
                e.path()
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default()
            })
        })
        .map(|name| Node::new(parent.clone(), name))
        .filter(|n| config.filter(n))
        .collect::<Vec<Node>>();

    nodes.sort_by(|a, b| config.sort(a, b));

    let focus_index = if let Some(focus) = focused_path {
        nodes
            .iter()
            .enumerate()
            .find(|(_, n)| n.relative_path() == &focus)
            .map(|(i, _)| i)
            .unwrap_or(0)
    } else {
        0
    };

    Ok(DirectoryBuffer::new(parent, nodes, focus_index))
}

pub fn explore_async(
    config: ExplorerConfig,
    parent: String,
    focused_path: Option<String>,
    tx: Sender<Task>,
) {
    thread::spawn(move || {
        explore_sync(config, parent.clone(), focused_path)
            .map(|dirbuf| {
                tx.send(Task::new(
                    MsgIn::Internal(InternalMsg::AddDirectory(parent, dirbuf)),
                    None,
                ))
                .unwrap_or_default();
            })
            .unwrap_or_else(|e| {
                tx.send(Task::new(
                    MsgIn::External(ExternalMsg::LogError(e.to_string())),
                    None,
                ))
                .unwrap_or_default();
            })
    });
}

pub fn explore_recursive_async(
    config: ExplorerConfig,
    parent: String,
    focused_path: Option<String>,
    tx: Sender<Task>,
) {
    let path = PathBuf::from(&parent);
    explore_async(config.clone(), parent, focused_path, tx.clone());
    if let Some(grand_parent) = path.parent() {
        explore_recursive_async(
            config,
            grand_parent.to_string_lossy().to_string(),
            path.file_name().map(|f| f.to_string_lossy().to_string()),
            tx,
        );
    }
}
