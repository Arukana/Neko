mod library;
pub mod err;

use ::git2;
use ::toml;

pub use self::err::{CompositerError, Result};

use self::library::Library;
use std::collections::VecDeque;
use std::env;
use std::io::{self, Read};
use std::fs;
use std::fs::File;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process;

/// The default capacity of heap.
const SPEC_CAPACITY: usize = 10;

/// The shared library extention.
#[cfg(any(target_os = "linux", target_os = "android"))]
const SPEC_LIB_EXT: &'static str = "so";
#[cfg(any(target_os = "bitrig", target_os = "dragonfly",
          target_os = "freebsd", target_os = "ios", target_os = "macos",
          target_os = "netbsd", target_os = "openbsd"))]
const SPEC_LIB_EXT: &'static str = "dylib";

/// The default priority of call.
const SPEC_PRIORITY: i64 = 0i64;
/// The name of priority label.
const SPEC_PRIORITY_NAME: &'static str = "priority";
/// The first directory.
const SPEC_ROOT: &'static str = ".neko";
/// The sub-directory git.
const SPEC_SUBD_GIT: &'static str = "git";
/// The sub-directory lib.
const SPEC_SUBD_LIB: &'static str = "lib";
/// The manigest NEKO file.
const SPEC_MANIFEST: &'static str = "Neko.toml";

/// The struct `Compositer` is a heap of a double tuple
/// of a dynamic libraries and a priority order.
#[derive(Debug)]
pub struct Compositer(VecDeque<Library>);

impl Compositer {

    /// The constructor `default` returns a Compositer prepared with
    /// the libraries from the root directory.
    pub fn new() -> Result<Self> {
        let mut compositer: Compositer =
        Compositer(VecDeque::with_capacity(SPEC_CAPACITY));

        match compositer.get_git() {
            Err(why) => Err(why),
            Ok(rep) => {
                match fs::read_dir(rep) {
                    Err(why) => Err(CompositerError::ReadDirGit(why)),
                    Ok(entries) => {
                        entries.filter_map(|repository|
                            repository.ok()
                        ).all(|entry|
                            compositer.mount(&entry.path(), None).is_ok()
                        );
                        Ok(compositer)
                    },
                }
            },
        }
    }

    /// The accessor method `into_inner` returning the Library Vector.
    pub fn into_inner(self) -> VecDeque<Library> {
        self.0
    }

    /// The accessor method `get_git` returns the git sub-directory.
    pub fn get_git(&self) -> Result<PathBuf> {
        if let Some(mut path) = env::home_dir() {
            path.push(SPEC_ROOT);
            path.push(SPEC_SUBD_GIT);
            if let Some(why) = fs::create_dir_all(&path).err() {
                if why.kind() == io::ErrorKind::AlreadyExists {
                    Ok(path)
                } else {
                    Err(CompositerError::MkDirGit(why))
                }
            } else {
                Ok(path)
            }
        } else {
            Err(CompositerError::Home)
        }
    }

    /// The accessor method `get_lib` returns the lib sub-directory.
    pub fn get_lib(&self) -> Result<PathBuf> {
        if let Some(mut path) = env::home_dir() {
            path.push(SPEC_ROOT);
            path.push(SPEC_SUBD_LIB);
            if let Some(why) = fs::create_dir_all(&path).err() {
                if why.kind() == io::ErrorKind::AlreadyExists {
                    Ok(path)
                } else {
                    Err(CompositerError::MkDirLib(why))
                }
            } else {
                Ok(path)
            }
        } else {
            Err(CompositerError::Home)
        }
    }

    /// The accessor method `git_with_lib` returns a couple
    /// of `git` and `lib` sub-repositories.
    pub fn git_with_lib(
        &mut self,
    ) -> Result<(PathBuf, PathBuf)> {
        match (self.get_git(), self.get_lib()) {
            (Err(why), _) | (_, Err(why)) => Err(why),
            (Ok(git), Ok(lib)) => Ok((git, lib)),
        }
    }

    /// The accessor method `get_manifest` returns a dictionary.
    pub fn get_manifest (
        &self,
        repository: &PathBuf
    ) -> Result<toml::Table> {
        match File::open(repository.join(SPEC_MANIFEST)) {
            Err(why) => Err(CompositerError::OpenDirLib(why)),
            Ok(mut descripter) => {
                let mut buffer = String::new();
                match descripter.read_to_string(&mut buffer) {
                    Err(why) => Err(CompositerError::ReadManifest(why)),
                    Ok(_) => {
                        if let Some(table) = toml::Parser::new(&buffer)
                                                          .parse() {
                            Ok(table)
                        } else {
                            Err(CompositerError::ParseManifest)
                        }
                    }
                }
            },
        }
    }

    /// The method `mount` adds a new library to the heap's compositer.
    pub fn mount<S: AsRef<OsStr>>(
        &mut self,
        libraryname: &S,
        priority: Option<i64>,
    ) -> Result<()> {
        self.git_with_lib().and_then(|(git, lib)|
            match self.unmount(libraryname) {
                Ok(_) | Err(CompositerError::UnmountPosition) => {
                    let source: PathBuf = PathBuf::from(libraryname);
                    self.get_manifest(&git.join(&source)).and_then(|table|
                        if let Some(priority) = priority.or(
                            parse_number!(table)
                        ) {
                            match Library::new(
                                lib.join(&source)
                                    .with_extension(SPEC_LIB_EXT),
                                priority
                            ) {
                                Err(why) => Err(CompositerError::Mount(why)),
                                Ok(dy) => {
                                    dy.start();
                                    Ok(self.0.push_back(dy))
                                },
                            }
                        } else {
                            Err(CompositerError::ParseInteger)
                        }
                    )
                },
                Err(why) => Err(why),
            }
        )
    }

    /// The method `unmount` removes library from the queue.
    pub fn unmount<S: AsRef<OsStr>>(
        &mut self, libraryname: S
    ) -> Result<Library> {
        if let Some(index) = self.0.iter().position(|ref s|
            s.as_path_buf().file_stem().eq(&Some(libraryname.as_ref()))
        ) {
            if let Some(removed) = self.0.remove(index) {
                Ok(removed)
            } else {
                Err(CompositerError::UnmountRemove)
            }
        } else {
            Err(CompositerError::UnmountPosition)
        }
    }

    /// The method `build` makes and adds a dynamic library
    /// to SPEC_MANIFEST's destination.
    pub fn build<S: AsRef<Path>>(
        &self, source: &PathBuf, sub: &S, lib: &str
    ) -> Result<()> {
        match self.get_lib() {
            Err(why) => Err(why),
            Ok(dest) => {
                match process::Command::new("make")
                              .current_dir(source.as_path())
                              .status() {
                    Err(why) => Err(CompositerError::BuildCommand(why)),
                    Ok(status) => if status.success() {
                        fs::rename(
                            source.join(lib)
                                  .with_extension(SPEC_LIB_EXT),
                            dest.join(sub)
                                .with_extension(SPEC_LIB_EXT)
                        ).or_else(|why: io::Error|
                                  Err(CompositerError::MvFail(why))
                        )
                    } else {
                        Err(CompositerError::BuildExit(status))
                    },
                }
            },
        }
    }
/*
    pub fn dependency(&self, repository: &PathBuf) -> Result<()> {
        match self.get_manifest(repository) {
            Err(why) => Err(why),
            Ok(table) => {
                println!("{:?}", table);
                Ok(())
            },
        }
    }
*/
    /// The methodd `install` clones and makes a dynamic library from repository
    /// and recursive call the dependencies.
    pub fn install(&mut self, repo: &str, mount: bool) -> Result<()> {
        self.get_git().and_then(|git|
            if let Some((sub, _, lib)) = parse_name!(repo) {
                let dest: PathBuf = git.join(&sub);
                if dest.exists() {
                    Err(CompositerError::InstallExists)
                } else {
                    match git2::Repository::clone(&repo, &dest) {
                        Err(why) => Err(CompositerError::InstallClone(why)),
                        Ok(_) => {
                            self.build(&dest, &sub, &lib).and(
                                if mount {
                                    self.mount(&sub, None)
                                } else {
                                    Ok(())
                                }
                            )/*.and(
                                self.dependency(&repo)
                            )*/
                        },
                    }
                }
            }
            else {
                Err(CompositerError::InstallFormat)
            }
        )
    }

    fn reset(&self, repo: &git2::Repository, object: &git2::Object) -> Result<()> {
        match repo.reset(
            object, git2::ResetType::Hard, None
        ) {
            Err(why) => Err(CompositerError::UpdateRepReset(why)),
            Ok(_) => Ok(()),
        }
    }

    fn update_from_master(&self, repo: &git2::Repository) -> Result<()> {
        match repo.find_branch(
          "master", git2::BranchType::Local
        ) {
            Err(why) => Err(CompositerError::UpdateRepBranch(why)),
            Ok(branch) => {
                match branch.get().target() {
                    None => Err(CompositerError::UpdateRepBranchId),
                    Some(id) => {
                        match repo.find_object(id, None) {
                            Err(why) => Err(CompositerError::UpdateRepObject(why)),
                            Ok(obj) => self.reset(repo, &obj),
                        }
                    },
                }
            },
        }
    }

    pub fn update(&mut self, repo: &str) -> Result<()> {
      self.get_git().and_then(|git|
        match git2::Repository::open(&git.join(&repo)) {
          Err(why) => Err(CompositerError::UpdateRepOpen(why)),
          Ok(rep) => {
            match rep.find_remote("origin") {
              Err(why) => Err(CompositerError::UpdateRepOrigin(why)),
              Ok(mut remote) => {
                if let Some(why) = remote.fetch(
                  &["refs/heads/*:refs/heads/*"], None, None
                ).err() {
                  Err(CompositerError::UpdateRepFetch(why))
                } else {
                    self.update_from_master(&rep)
                }
              },
            }
          },
        }
      )
    }

    /// The method `uninstall` removes library from the filesystem with
    /// the source.
    pub fn uninstall<S: AsRef<OsStr>>(
        &mut self,
        name: &S
    ) -> Result<()> {
        match self.unmount(name) {
            Ok(_) | Err(CompositerError::UnmountPosition) => {
                let path: PathBuf = PathBuf::from(name);
                match (self.get_git(), self.get_lib()) {
                    (Ok(git), Ok(lib)) => {
                        if let Err(why) = fs::remove_file(
                            lib.join(&path).with_extension(SPEC_LIB_EXT)
                        ) {
                            Err(CompositerError::RmFile(why))
                        } else {
                            if let Err(why) = fs::remove_dir_all(git.join(&path)) {
                                Err(CompositerError::RmDir(why))
                            } else {
                                Ok(())
                            }
                        }
                    },
                    (Err(why), _) | (_, Err(why)) => Err(why),
                }
            },
            Err(why) => Err(why),
        }
    }
}
