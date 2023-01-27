use anyhow::{bail, Result};
use lazy_static::lazy_static;
use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref HOME: Option<String> = std::env::var("HOME").ok();
}

// Stolen from https://github.com/Manishearth/pathdiff/blob/master/src/lib.rs
pub fn diff<P, B>(path: P, base: B) -> Result<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Ok(PathBuf::from(path))
        } else {
            let path = path.to_string_lossy();
            bail!("{path}: is not absolute")
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
                (Some(_), Some(b)) if b == Component::ParentDir => {
                    let path = path.to_string_lossy();
                    let base = base.to_string_lossy();
                    bail!("{base} is not a parent of {path}")
                }
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
        Ok(comps.iter().map(|c| c.as_os_str()).collect())
    }
}

pub fn relative_to<P, B>(path: P, base: Option<B>) -> Result<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let base = match base {
        Some(base) => PathBuf::from(base.as_ref()),
        None => std::env::current_dir()?,
    };

    let diff = diff(path, base)?;

    if diff.to_str() == Some("") {
        Ok(".".into())
    } else {
        Ok(diff)
    }
}

pub fn shortened<P, B>(path: P, base: Option<B>) -> Result<String>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let pathstring = path.to_string_lossy().to_string();
    let pathstr = pathstring.as_str();
    let relative = relative_to(path, base)?;

    let relative = if relative.to_str() == Some(".") {
        match (path.parent(), path.file_name()) {
            (Some(_), Some(name)) => {
                let name = name.to_string_lossy();
                format!("../{name}")
            }
            (_, _) => relative.to_string_lossy().to_string(),
        }
    } else if relative.to_str() == Some("..") {
        match (path.parent(), path.file_name()) {
            (Some(parent), Some(name)) => {
                let name = name.to_string_lossy();
                if parent.parent().is_some() {
                    format!("../../{name}")
                } else {
                    relative.to_string_lossy().to_string()
                }
            }
            (_, _) => relative.to_string_lossy().to_string(),
        }
    } else {
        relative.to_string_lossy().to_string()
    };

    let res = HOME
        .as_ref()
        .and_then(|h| pathstr.strip_prefix(h).map(|p| format!("~{p}")))
        .unwrap_or(pathstring);

    if relative.len() < res.len() {
        Ok(relative)
    } else {
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_to_pwd() {
        let path = std::env::current_dir().unwrap();
        let relative = relative_to(path, Option::<String>::None).unwrap();
        assert_eq!(relative, PathBuf::from("."));
    }

    #[test]
    fn test_relative_to_parent() {
        let path = std::env::current_dir().unwrap();
        let path = path.parent().unwrap();

        let relative = relative_to(path, Option::<String>::None).unwrap();
        assert_eq!(relative, PathBuf::from(".."));
    }

    #[test]
    fn test_relative_to_file() {
        let path = std::env::current_dir().unwrap().join("foo").join("bar");
        let relative = relative_to(path, Option::<String>::None).unwrap();
        assert_eq!(relative, PathBuf::from("foo/bar"));
    }

    #[test]
    fn test_relative_to_root() {
        let relative = relative_to("/foo", Some("/")).unwrap();
        assert_eq!(relative, PathBuf::from("foo"));

        let relative = relative_to("/", Some("/")).unwrap();
        assert_eq!(relative, PathBuf::from("."));

        let relative = relative_to("/", Some("/foo")).unwrap();
        assert_eq!(relative, PathBuf::from(".."));
    }

    #[test]
    fn test_relative_to_base() {
        let path = "/some/directory";
        let base = "/another/foo/bar";
        let relative = relative_to(path, Some(base)).unwrap();
        assert_eq!(relative, PathBuf::from("../../../some/directory"));
    }

    #[test]
    fn test_shorthand_to_home() {
        let path = HOME.as_ref().unwrap();

        let res = shortened(path, Option::<String>::None).unwrap();
        assert_eq!(res, "~");
    }

    #[test]
    fn test_shorthand_to_base() {
        let path = "/present/working/directory";
        let base = "/present/foo/bar";

        let res = shortened(path, Some(base)).unwrap();
        assert_eq!(res, "../../working/directory");
    }

    #[test]
    fn test_shorthand_to_pwd() {
        let path = "/present/working/directory";

        let res = shortened(&path, Some(&path)).unwrap();
        assert_eq!(res, "../directory");
    }

    #[test]
    fn test_shorthand_to_parent() {
        let path = "/present/working";
        let base = "/present/working/directory";

        let res = shortened(&path, Some(&base)).unwrap();
        assert_eq!(res, "../../working");
    }

    #[test]
    fn test_shorthand_to_root() {
        let res = shortened("/", Some("/")).unwrap();
        assert_eq!(res, "/");

        let res = shortened("/foo", Some("/")).unwrap();
        assert_eq!(res, "foo");

        let res = shortened("/", Some("/foo")).unwrap();
        assert_eq!(res, "/");
    }
}
