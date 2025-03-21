use crate::dirs;
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
pub use snailquote::escape;
use std::path::{Component, Path, PathBuf};

lazy_static! {
    pub static ref HOME: Option<PathBuf> = dirs::home_dir();
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
                (Some(a), Some(Component::CurDir)) => comps.push(a),
                (Some(_), Some(Component::ParentDir)) => {
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RelativityConfig<B: AsRef<Path>> {
    base: Option<B>,
    with_prefix_dots: Option<bool>,
    without_suffix_dots: Option<bool>,
    without_tilde: Option<bool>,
}

impl<B: AsRef<Path>> RelativityConfig<B> {
    pub fn with_base(mut self, base: B) -> Self {
        self.base = Some(base);
        self
    }

    pub fn with_prefix_dots(mut self) -> Self {
        self.with_prefix_dots = Some(true);
        self
    }

    pub fn without_suffix_dots(mut self) -> Self {
        self.without_suffix_dots = Some(true);
        self
    }

    pub fn without_tilde(mut self) -> Self {
        self.without_tilde = Some(true);
        self
    }
}

pub fn relative_to<P, B>(
    path: P,
    config: Option<&RelativityConfig<B>>,
) -> Result<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = match config.and_then(|c| c.base.as_ref()) {
        Some(base) => PathBuf::from(base.as_ref()),
        None => std::env::current_dir()?,
    };

    let diff = diff(path, base)?;

    let relative = if diff.to_str() == Some("") {
        ".".into()
    } else {
        diff
    };

    let relative = if config.and_then(|c| c.with_prefix_dots).unwrap_or(false)
        && !relative.starts_with(".")
        && !relative.starts_with("..")
    {
        PathBuf::from(".").join(relative)
    } else {
        relative
    };

    let relative = if !config.and_then(|c| c.without_suffix_dots).unwrap_or(false) {
        relative
    } else if relative.ends_with(".") {
        match (path.parent(), path.file_name()) {
            (Some(_), Some(name)) => PathBuf::from("..").join(name),
            (_, _) => relative,
        }
    } else if relative.ends_with("..") {
        match (path.parent(), path.file_name()) {
            (Some(parent), Some(name)) => {
                if parent.parent().is_some() {
                    relative.join("..").join(name)
                } else {
                    // always prefer absolute path if it's a child of the root directory
                    // to guarantee that the basename is included
                    path.into()
                }
            }
            (_, _) => relative,
        }
    } else {
        relative
    };

    Ok(relative)
}

pub fn shorten<P, B>(path: P, config: Option<&RelativityConfig<B>>) -> Result<String>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let pathstring = path.to_string_lossy().to_string();
    let relative = relative_to(path, config)?;

    let relative = relative.to_string_lossy().to_string();

    if config.and_then(|c| c.without_tilde).unwrap_or(false) {
        return if relative.len() < pathstring.len() {
            Ok(relative)
        } else {
            Ok(pathstring)
        };
    }

    let fromhome = HOME
        .as_ref()
        .and_then(|h| {
            path.strip_prefix(h).ok().map(|p| {
                if p.to_str() == Some("") {
                    "~".into()
                } else {
                    PathBuf::from("~").join(p).to_string_lossy().to_string()
                }
            })
        })
        .unwrap_or(pathstring);

    if relative.len() < fromhome.len() {
        Ok(relative)
    } else {
        Ok(fromhome)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type Config<'a> = Option<&'a RelativityConfig<String>>;

    const NONE: Config = Config::None;

    fn default<'a>() -> RelativityConfig<&'a str> {
        Default::default()
    }

    #[test]
    fn test_relative_to_pwd() {
        let path = std::env::current_dir().unwrap();

        let relative = relative_to(&path, NONE).unwrap();
        assert_eq!(relative, PathBuf::from("."));

        let relative = relative_to(&path, Some(&default().with_prefix_dots())).unwrap();
        assert_eq!(relative, PathBuf::from("."));

        let relative =
            relative_to(&path, Some(&default().without_suffix_dots())).unwrap();
        assert_eq!(
            relative,
            PathBuf::from("..").join(path.file_name().unwrap())
        );

        let relative = relative_to(
            &path,
            Some(&default().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(
            relative,
            PathBuf::from("..").join(path.file_name().unwrap())
        );
    }

    #[test]
    fn test_relative_to_parent() {
        let path = std::env::current_dir().unwrap().join("docs");
        let parent = path.parent().unwrap();

        let base = default().with_base(path.to_str().unwrap());

        let relative = relative_to(parent, Some(&base)).unwrap();
        assert_eq!(relative, PathBuf::from(".."));

        let relative =
            relative_to(parent, Some(&base.clone().with_prefix_dots())).unwrap();
        assert_eq!(relative, PathBuf::from(".."));

        let relative =
            relative_to(parent, Some(&base.clone().without_suffix_dots())).unwrap();
        assert_eq!(
            relative,
            PathBuf::from("../..").join(parent.file_name().unwrap())
        );

        let relative = relative_to(
            parent,
            Some(&base.clone().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(
            relative,
            PathBuf::from("../..").join(parent.file_name().unwrap())
        );
    }

    #[test]
    fn test_relative_to_file() {
        let path = std::env::current_dir().unwrap().join("foo").join("bar");

        let relative = relative_to(&path, NONE).unwrap();
        assert_eq!(relative, PathBuf::from("foo/bar"));

        let relative = relative_to(&path, Some(&default().with_prefix_dots())).unwrap();
        assert_eq!(relative, PathBuf::from("./foo/bar"));

        let relative = relative_to(
            &path,
            Some(&default().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(relative, PathBuf::from("./foo/bar"));
    }

    #[test]
    fn test_relative_to_root() {
        let relative = relative_to("/foo", Some(&default().with_base("/"))).unwrap();
        assert_eq!(relative, PathBuf::from("foo"));

        let relative = relative_to(
            "/foo",
            Some(
                &default()
                    .with_base("/")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(relative, PathBuf::from("./foo"));

        let relative = relative_to("/", Some(&default().with_base("/"))).unwrap();
        assert_eq!(relative, PathBuf::from("."));

        let relative = relative_to(
            "/",
            Some(
                &default()
                    .with_base("/")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(relative, PathBuf::from("."));

        let relative = relative_to("/", Some(&default().with_base("/foo"))).unwrap();
        assert_eq!(relative, PathBuf::from(".."));

        let relative = relative_to(
            "/",
            Some(
                &default()
                    .with_base("/foo")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(relative, PathBuf::from(".."));
    }

    #[test]
    fn test_relative_to_base() {
        let path = "/some/directory";
        let base = "/another/foo/bar";

        let relative = relative_to(path, Some(&default().with_base(base))).unwrap();
        assert_eq!(relative, PathBuf::from("../../../some/directory"));

        let relative = relative_to(
            path,
            Some(
                &default()
                    .with_base(base)
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(relative, PathBuf::from("../../../some/directory"));
    }

    #[test]
    fn test_shorten_home() {
        let path = HOME.as_ref().unwrap();

        let res = shorten(path, NONE).unwrap();
        assert_eq!(res, "~");

        let res = shorten(
            path,
            Some(&default().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(res, "~");

        let res = shorten(
            path,
            Some(&default().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(res, "~");

        let res = shorten(path.join("foo"), NONE).unwrap();
        assert_eq!(res, "~/foo");

        let res = shorten(
            path.join("foo"),
            Some(&default().with_prefix_dots().without_suffix_dots()),
        )
        .unwrap();
        assert_eq!(res, "~/foo");

        let res = shorten(format!("{}foo", path.to_string_lossy()), NONE).unwrap();
        assert_ne!(res, "~/foo");
        assert_eq!(res, format!("{}foo", path.to_string_lossy()));
    }

    #[test]
    fn test_shorten_base() {
        let path = "/present/working/directory";
        let base = "/present/foo/bar";

        let res = shorten(path, Some(&default().with_base(base))).unwrap();
        assert_eq!(res, "../../working/directory");

        let res = shorten(
            path,
            Some(
                &default()
                    .with_base(base)
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "../../working/directory");
    }

    #[test]
    fn test_shorten_pwd() {
        let path = "/present/working/directory";

        let res = shorten(path, Some(&default().with_base(path))).unwrap();
        assert_eq!(res, ".");

        let res = shorten(
            path,
            Some(
                &default()
                    .with_base(path)
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "../directory");
    }

    #[test]
    fn test_shorten_parent() {
        let path = "/present/working";
        let base = "/present/working/directory";

        let res = shorten(path, Some(&default().with_base(base))).unwrap();
        assert_eq!(res, "..");

        let res = shorten(
            path,
            Some(
                &default()
                    .with_base(base)
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "../../working");
    }

    #[test]
    fn test_shorten_root() {
        let res = shorten("/", Some(&default().with_base("/"))).unwrap();
        assert_eq!(res, "/");

        let res = shorten(
            "/",
            Some(
                &default()
                    .with_base("/")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "/");

        let res = shorten("/foo", Some(&default().with_base("/"))).unwrap();
        assert_eq!(res, "foo");

        let res = shorten(
            "/foo",
            Some(
                &default()
                    .with_base("/")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "/foo");

        let res = shorten(
            "/",
            Some(
                &default()
                    .with_base("/foo")
                    .with_prefix_dots()
                    .without_suffix_dots(),
            ),
        )
        .unwrap();
        assert_eq!(res, "/");
    }

    #[test]
    fn test_path_escape() {
        let text = "foo".to_string();
        assert_eq!(escape(&text), "foo");

        let text = "foo bar".to_string();
        assert_eq!(escape(&text), "'foo bar'");

        let text = "foo\nbar".to_string();
        assert_eq!(escape(&text), "\"foo\\nbar\"");

        let text = "foo$bar".to_string();
        assert_eq!(escape(&text), "'foo$bar'");

        let text = "foo'$\n'bar".to_string();
        assert_eq!(escape(&text), "\"foo'\\$\\n'bar\"");

        let text = "a'b\"c".to_string();
        assert_eq!(escape(&text), "\"a'b\\\"c\"");
    }
}
