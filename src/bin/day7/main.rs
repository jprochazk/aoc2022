use std::path::{Path, PathBuf};

use vfs::PathClean;

fn main() -> anyhow::Result<()> {
  let input = include_str!("input.txt").trim();

  let mut fs = vfs::Fs::new();
  let mut cwd = PathBuf::from("/");

  let mut lines = input.split('\n').peekable();
  while let Some(line) = lines.next() {
    if line.starts_with('$') {
      match Command::parse(line.split_once(' ').unwrap().1) {
        Command::ChangeDir(path) => {
          cwd = cwd.join(path).clean();
        }
        Command::List => {
          while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
            let (info, path) = lines.next().unwrap().split_once(' ').unwrap();
            let path = cwd.join(path).clean();
            match info {
              "dir" => fs.create_dir(&path)?,
              _ => fs.create_file(&path, info.parse()?)?,
            };
          }
        }
      }
    }
  }

  let mut sum = 0;
  for entry in fs.entries() {
    use vfs::Entry::*;

    let Dir(_) = entry else {
      continue;
    };
    let size = fs.size(entry.path())?;
    if size > 100_000 {
      continue;
    }

    sum += size;
  }

  println!("Day 1 part 1 answer: {}", sum);

  let total = fs.size("/")?;
  let unused = 70_000_000 - total;
  let required = 30_000_000 - unused;

  let mut smallest = usize::MAX;
  for entry in fs.entries() {
    use vfs::Entry::*;

    let Dir(_) = entry else {
      continue;
    };
    let size = fs.size(entry.path())?;
    if size >= required && size < smallest {
      smallest = size;
      continue;
    }
  }

  println!("Day 1 part 2 answer: {}", smallest);

  Ok(())
}

enum Command<'a> {
  ChangeDir(&'a Path),
  List,
}

impl<'a> Command<'a> {
  fn parse(s: &'a str) -> Self {
    use Command::*;
    if s.starts_with("cd") {
      let (_, args) = s.split_once(' ').unwrap();
      ChangeDir(Path::new(args))
    } else if s.starts_with("ls") {
      List
    } else {
      panic!("unknown command `{s}`")
    }
  }
}

mod vfs {
  #![allow(dead_code)]

  use std::collections::BTreeMap;
  use std::fmt;
  use std::path::{Component, Path, PathBuf};

  use slotmap::SlotMap;
  use thiserror::Error;

  slotmap::new_key_type! {
    pub struct EntryId;
  }

  pub struct Fs {
    storage: SlotMap<EntryId, Entry>,
    index: BTreeMap<PathBuf, EntryId>,
    root: EntryId,
  }

  impl Fs {
    pub fn new() -> Self {
      let mut storage = SlotMap::with_key();
      let root = storage.insert(Entry::dir("/"));
      let index = BTreeMap::from([("/".into(), root)]);
      Self {
        storage,
        root,
        index,
      }
    }

    pub fn root(&self) -> EntryId {
      self.root
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<&Entry> {
      let path = path.as_ref();
      self
        .index
        .get(path)
        .and_then(|id| self.storage.get(*id))
        .ok_or_else(|| Error::FileNotFound(path.into()))
    }

    pub fn open_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut Entry> {
      let path = path.as_ref();
      self
        .index
        .get(path)
        .and_then(|id| self.storage.get_mut(*id))
        .ok_or_else(|| Error::FileNotFound(path.into()))
    }

    pub fn create_file(&mut self, path: impl AsRef<Path>, size: usize) -> Result<()> {
      let path = path.as_ref();
      self.create(path, Entry::file(path, size))
    }

    pub fn create_dir(&mut self, path: impl AsRef<Path>) -> Result<()> {
      let path = path.as_ref();
      self.create(path, Entry::dir(path))
    }

    fn create(&mut self, path: impl AsRef<Path>, entry: Entry) -> Result<()> {
      let path = path.as_ref();

      if self.open(path).is_ok() {
        return Err(Error::FileExists(path.into()));
      }

      let Some(parent_path) = path.parent() else {
        return Err(Error::ParentNotFound(path.into()))
      };

      let id = self.storage.insert(entry);

      let Ok(parent) = self.open_mut(parent_path) else {
        self.storage.remove(id);
        return Err(Error::ParentNotFound(path.into()));
      };
      let Ok(parent) = parent.as_dir_mut() else {
        self.storage.remove(id);
        return Err(Error::NotDir(parent_path.into()));
      };

      parent.add(id);

      self.index.insert(path.into(), id);

      Ok(())
    }

    pub fn size(&self, path: impl AsRef<Path>) -> Result<usize> {
      match self.open(path)? {
        Entry::File(file) => Ok(file.size),
        Entry::Dir(dir) => {
          let mut sum = 0;
          for entry in dir.entries.iter().map(|id| self.storage.get(*id).unwrap()) {
            sum += self.size(entry.path())?;
          }
          Ok(sum)
        }
      }
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
      self.storage.iter().map(|(_, entry)| entry)
    }
  }

  #[derive(Debug)]
  pub enum Entry {
    File(File),
    Dir(Dir),
  }

  impl Entry {
    pub fn path(&self) -> &Path {
      match self {
        Entry::File(entry) => entry.path(),
        Entry::Dir(entry) => entry.path(),
      }
    }

    fn file(path: impl Into<PathBuf>, size: usize) -> Entry {
      Entry::File(File {
        path: path.into(),
        size,
      })
    }

    fn dir(path: impl Into<PathBuf>) -> Entry {
      Entry::Dir(Dir {
        path: path.into(),
        entries: vec![],
      })
    }

    pub fn as_dir(&self) -> Result<&Dir> {
      match self {
        Entry::File(e) => Err(Error::NotDir(e.path().into())),
        Entry::Dir(e) => Ok(e),
      }
    }

    pub fn as_dir_mut(&mut self) -> Result<&mut Dir> {
      match self {
        Entry::File(e) => Err(Error::NotDir(e.path().into())),
        Entry::Dir(e) => Ok(e),
      }
    }

    pub fn as_file(&self) -> Result<&File> {
      match self {
        Entry::File(e) => Ok(e),
        Entry::Dir(e) => Err(Error::NotFile(e.path().into())),
      }
    }

    pub fn as_file_mut(&mut self) -> Result<&mut File> {
      match self {
        Entry::File(e) => Ok(e),
        Entry::Dir(e) => Err(Error::NotFile(e.path().into())),
      }
    }
  }

  #[derive(Debug)]
  pub struct File {
    path: PathBuf,
    size: usize,
  }

  impl File {
    pub fn path(&self) -> &Path {
      self.path.as_path()
    }

    pub fn size(&self) -> usize {
      self.size
    }
  }

  #[derive(Debug)]
  pub struct Dir {
    path: PathBuf,
    entries: Vec<EntryId>,
  }

  impl Dir {
    pub fn path(&self) -> &Path {
      self.path.as_path()
    }

    pub fn add(&mut self, entry: EntryId) {
      self.entries.push(entry);
    }
  }

  #[derive(Debug, Error)]
  pub enum Error {
    #[error("`{0}` not found")]
    FileNotFound(PathBuf),
    #[error("`{0}` already exists")]
    FileExists(PathBuf),
    #[error("parent dir of `{0}` not found")]
    ParentNotFound(PathBuf),
    #[error("`{0}` is not a directory")]
    NotDir(PathBuf),
    #[error("`{0}` is not a file")]
    NotFile(PathBuf),
  }

  impl Error {
    pub fn path(&self) -> &Path {
      match self {
        Error::FileNotFound(path) => path,
        Error::FileExists(path) => path,
        Error::ParentNotFound(path) => path,
        Error::NotDir(path) => path,
        Error::NotFile(path) => path,
      }
    }
  }

  pub type Result<T> = std::result::Result<T, Error>;

  /// The Clean trait implements a `clean` method.
  pub trait PathClean {
    fn clean(&self) -> PathBuf;
  }

  /// PathClean implemented for `Path`
  impl PathClean for Path {
    fn clean(&self) -> PathBuf {
      clean(self)
    }
  }

  pub fn clean<P>(path: P) -> PathBuf
  where
    P: AsRef<Path>,
  {
    let mut out = Vec::new();

    for comp in path.as_ref().components() {
      match comp {
        Component::CurDir => (),
        Component::ParentDir => match out.last() {
          Some(Component::RootDir) => (),
          Some(Component::Normal(_)) => {
            out.pop();
          }
          None
          | Some(Component::CurDir)
          | Some(Component::ParentDir)
          | Some(Component::Prefix(_)) => out.push(comp),
        },
        comp => out.push(comp),
      }
    }

    if !out.is_empty() {
      out.iter().collect()
    } else {
      PathBuf::from(".")
    }
  }

  impl fmt::Display for Fs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fn fmt_inner(fs: &Fs, dir: &Dir, depth: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in dir.entries.iter() {
          let entry = fs.storage.get(*entry).ok_or(fmt::Error)?;
          let name = entry
            .path()
            .components()
            .last()
            .ok_or(fmt::Error)?
            .as_os_str()
            .to_string_lossy();

          writeln!(f, "{:>width$}|- {}", "", name, width = depth * 3)?;
          if let Entry::Dir(dir) = entry {
            fmt_inner(fs, dir, depth + 1, f)?;
          }
        }

        Ok(())
      }

      let root = self
        .storage
        .get(self.root())
        .ok_or(fmt::Error)?
        .as_dir()
        .map_err(|_| fmt::Error)?;

      writeln!(f, "<root>")?;
      fmt_inner(self, root, 0, f)?;

      Ok(())
    }
  }
}
