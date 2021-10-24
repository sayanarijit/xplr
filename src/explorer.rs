use crate::app::{DirectoryBuffer, ExplorerConfig, ExternalMsg, InternalMsg, MsgIn, Node, Task};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;

pub(crate) fn explore_sync(
    config: ExplorerConfig,
    parent: PathBuf,
    focused_path: Option<PathBuf>,
    fallback_focus: usize,
) -> Result<DirectoryBuffer> {
    let dirs = fs::read_dir(&parent)?;
    let mut nodes = dirs
        .filter_map(|d| {
            d.ok().map(|e| {
                e.path()
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default()
            })
        })
        .map(|name| Node::new(parent.to_string_lossy().to_string(), name))
        .filter(|n| config.filter(n))
        .collect::<Vec<Node>>();

    nodes.sort_by(|a, b| config.sort(a, b));

    let focus_index = if let Some(focus) = focused_path {
        let focus_str = focus.to_string_lossy().to_string();
        nodes
            .iter()
            .enumerate()
            .find(|(_, n)| n.relative_path == focus_str)
            .map(|(i, _)| i)
            .unwrap_or_else(|| fallback_focus.min(nodes.len().max(1) - 1))
    } else {
        0
    };

    Ok(DirectoryBuffer::new(
        parent.to_string_lossy().to_string(),
        nodes,
        focus_index,
    ))
}

pub(crate) fn explore_async(
    config: ExplorerConfig,
    parent: PathBuf,
    focused_path: Option<PathBuf>,
    fallback_focus: usize,
    tx_msg_in: Sender<Task>,
) {
    thread::spawn(move || {
        explore_sync(config, parent.clone(), focused_path, fallback_focus)
            .map(|buf| {
                tx_msg_in
                    .send(Task::new(
                        MsgIn::Internal(InternalMsg::SetDirectory(buf)),
                        None,
                    ))
                    .unwrap();
            })
            .unwrap_or_else(|e| {
                tx_msg_in
                    .send(Task::new(
                        MsgIn::External(ExternalMsg::LogError(e.to_string())),
                        None,
                    ))
                    .unwrap();
            })
    });
}

pub(crate) fn explore_recursive_async(
    config: ExplorerConfig,
    parent: PathBuf,
    focused_path: Option<PathBuf>,
    fallback_focus: usize,
    tx_msg_in: Sender<Task>,
) {
    explore_async(
        config.clone(),
        parent.clone(),
        focused_path,
        fallback_focus,
        tx_msg_in.clone(),
    );
    if let Some(grand_parent) = parent.parent() {
        explore_recursive_async(
            config,
            grand_parent.into(),
            parent.file_name().map(|p| p.into()),
            0,
            tx_msg_in,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explore_sync() {
        let config = ExplorerConfig::default();
        let path = PathBuf::from("/bin");

        let r = explore_sync(config, path, None, 0);

        assert!(r.is_ok());
    }

    #[test]
    fn test_failed_explore_sync() {
        let config = ExplorerConfig::default();
        let path = PathBuf::from("/there/is/no/path");

        let r = explore_sync(config, path, None, 0);

        assert!(r.is_err());
    }

    fn extract_dirbuf_from_msg(msg: MsgIn) -> DirectoryBuffer {
        assert!(matches!(msg, MsgIn::Internal(_)));

        let msgin = match msg {
            MsgIn::Internal(m) => m,
            _ => panic!(),
        };

        assert!(matches!(msgin, InternalMsg::SetDirectory(_)));

        match msgin {
            InternalMsg::SetDirectory(dbuf) => dbuf,
            _ => panic!(),
        }
    }

    use std::sync::mpsc;

    #[test]
    fn test_explore_async() {
        let config = ExplorerConfig::default();
        let path = PathBuf::from("/usr/bin");
        let (tx_msg_in, rx_msg_in) = mpsc::channel();

        explore_async(config, path, None, 0, tx_msg_in.clone());

        let task = rx_msg_in.recv().unwrap();
        let dbuf = extract_dirbuf_from_msg(task.msg);

        assert_eq!(dbuf.parent, "/usr/bin");

        drop(tx_msg_in);
        assert!(rx_msg_in.recv().is_err());
    }

    //XXX: explore_recursive_async() generates messages with random order.
    // Discussing on GitHub (https://github.com/sayanarijit/xplr/issues/372)
    //#[test]
    //fn test_explore_recursive_async() {
    //    let config = ExplorerConfig::default();
    //    let path = PathBuf::from("/usr/bin");
    //    let (tx_msg_in, rx_msg_in) = mpsc::channel();

    //    explore_recursive_async(config, path, None, 0, tx_msg_in.clone());

    //    let mut task = rx_msg_in.recv().unwrap();
    //    let mut dbuf = extract_dirbuf_from_msg(task.msg);

    //    assert_eq!(dbuf.parent, "/");

    //    task = rx_msg_in.recv().unwrap();
    //    dbuf = extract_dirbuf_from_msg(task.msg);

    //    assert_eq!(dbuf.parent, "/usr");

    //    task = rx_msg_in.recv().unwrap();
    //    dbuf = extract_dirbuf_from_msg(task.msg);

    //    assert_eq!(dbuf.parent, "/usr/bin");

    //    drop(tx_msg_in);
    //    assert!(rx_msg_in.recv().is_err());
    //}
}
