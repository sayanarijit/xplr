// Stolen from https://github.com/Manishearth/pathdiff/blob/master/src/lib.rs

use std::path::*;

pub fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}

pub fn relative_path<P, B>(path: P, base: Option<B>) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let base = match base {
        Some(base) => Some(PathBuf::from(base.as_ref())),
        None => match std::env::current_dir() {
            Ok(base) => Some(base),
            Err(_) => None,
        },
    };

    base.and_then(|base| diff_paths(path, base))
}

pub fn relative_path_as_string(path: String, base: Option<String>) -> Option<String> {
    relative_path(path, base)
        .map(|path| path.to_string_lossy().to_string())
        .map(|path| {
            if path.is_empty() {
                ".".to_string()
            } else {
                path
            }
        })
}
