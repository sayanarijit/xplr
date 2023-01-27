use lazy_static::lazy_static;
use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref HOME: Option<String> = std::env::var("HOME").ok();
}

// Stolen from https://github.com/Manishearth/pathdiff/blob/master/src/lib.rs
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

pub fn relative_path_as_string<P, B>(path: P, base: Option<B>) -> Option<String>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    relative_path(path, base).map(|path| {
        let path = path.to_string_lossy().to_string();
        if path.is_empty() {
            ".".to_string()
        } else {
            path
        }
    })
}

pub fn path_shorthand<P, B>(path: P, base: Option<B>) -> String
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let pathstring = path.as_ref().to_string_lossy().to_string();
    let pathstr = pathstring.as_str();
    let relative = relative_path_as_string(path, base);

    let shortened = HOME
        .as_ref()
        .and_then(|h| pathstr.strip_prefix(h).map(|p| format!("~{p}")))
        .unwrap_or(pathstring);

    let shorthand = match relative {
        Some(relative) => {
            if relative.len() < shortened.len() {
                relative
            } else {
                shortened
            }
        }
        None => shortened,
    };

    shorthand
}
