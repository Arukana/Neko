use std::error::Error;
use std::fmt;
use std::io;
use std::env;
use std::process;

use ::git2;

use super::library::LibraryError;

pub type Result<T> = ::std::result::Result<T, CompositerError>;

/// The enum `CompositerError` defines the possible errors
/// from constructor Compositer.
#[derive(Debug)]
pub enum CompositerError {
    /// The directory can't be moved.
    MvFail(io::Error),
    /// Can't remove the file.
    RmFile(io::Error),
    /// Can't remove the directory.
    RmDir(io::Error),
    /// Can't create the `git` sub-directory.
    MkDirGit(io::Error),
    /// Can't create the `lib` sub-directory.
    MkDirLib(io::Error),
    /// Can't read the `git` sub-directory.
    ReadDirGit(io::Error),
    /// Can't read the `lib` sub-directory.
    ReadDirLib(io::Error),
    /// Can't open the `lib` sub-directory.
    OpenDirLib(io::Error),
    /// Can't run the command.
    BuildCommand(io::Error),
    /// Can't read the `manifest` Neko.toml file.
    ReadManifest(io::Error),
    /// Can't clone the repository.
    InstallClone(git2::Error),
    /// Can't update the repository.
    UpdateRepOpen(git2::Error),
    /// Can't found the origin from repository.
    UpdateRepOrigin(git2::Error),
    /// Can't fetch them repository.
    UpdateRepFetch(git2::Error),
    /// Can't found the branch from repository.
    UpdateRepBranch(git2::Error),
    /// Can't found the object from target identifiant.
    UpdateRepObject(git2::Error),
    /// Can't reset the repository.
    UpdateRepReset(git2::Error),
    /// Can't mount the dynamic library.
    Mount(LibraryError),
    /// The build haven't exited with success.
    BuildExit(process::ExitStatus),
    /// Can't get the target identifiant from branch.
    UpdateRepBranchId,
    /// Can't found the NEKO_PATH environement variable.
    NekoPath,
    /// Can't found the position.
    UnmountPosition,
    /// Can't remove the index.
    UnmountRemove,
    /// Can't parse the `manifest` Neko.toml file.
    ParseManifest,
    /// Can't parse a integer from the table.
    ParseInteger,
    /// The lib git haven't a valid format.
    InstallFormat,
    /// The dynamic library as already a repository.
    InstallExists,
    Io(io::Error),
}

impl fmt::Display for CompositerError {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Error for CompositerError {
    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            CompositerError::MvFail(_) => "The directory can't be moved.",
            CompositerError::RmFile(_) => "Can't remove the file.",
            CompositerError::RmDir(_) => "Can't remove the directory.",
            CompositerError::MkDirGit(_) => "Can't create the `git` sub-directory.",
            CompositerError::MkDirLib(_) => "Can't create the `Lib` sub-directory.",
            CompositerError::ReadDirGit(_) => "Can't read the `git` sub-directory.",
            CompositerError::ReadDirLib(_) => "Can't read the `Lib` sub-directory.",
            CompositerError::OpenDirLib(_) => "Can't open the `lib` sub-directory.",
            CompositerError::ReadManifest(_) => {
                "Can't read the `manifest` Neko.toml\
                                             file."
            }
            CompositerError::Mount(_) => "Can't mount the dynamic library.",
            CompositerError::InstallClone(_) => "Can't clone the repository",
            CompositerError::UpdateRepOpen(_) => "Can't update the repository.",
            CompositerError::UpdateRepOrigin(_) => {
                "Can't found the origin from\
                                                repository."
            }
            CompositerError::UpdateRepFetch(_) => "Can't fetch them repository.",
            CompositerError::UpdateRepBranch(_) => {
                "Can't found the branch from\
                                               repository."
            }
            CompositerError::UpdateRepBranchId => {
                "Can't get the target\
                                                  identifiant from branch."
            }
            CompositerError::UpdateRepObject(_) => {
                "Can't found the object from\
                                                target identifiant."
            }
            CompositerError::UpdateRepReset(_) => "Can't reset the repository.",
            CompositerError::BuildCommand(_) => "Can't run the command.",
            CompositerError::BuildExit(_) => "The build haven't exited with success.",
            CompositerError::NekoPath => "Can't found the $NEKO_PATH environement variable.",
            CompositerError::ParseManifest => {
                "Can't parse the `manifest` Neko.toml\
                                           file."
            }
            CompositerError::ParseInteger => "Can't parse a integer from the table.",
            CompositerError::UnmountPosition => "Can't found the position.",
            CompositerError::UnmountRemove => "Can't remove the index.",
            CompositerError::InstallFormat => "The git link haven't a valid format",
            CompositerError::InstallExists => {
                "The dynamic library as already a\
                                           repository."
            },
            CompositerError::Io(ref why) => why.description(),
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            CompositerError::MvFail(ref why) |
            CompositerError::RmFile(ref why) |
            CompositerError::RmDir(ref why) |
            CompositerError::MkDirGit(ref why) |
            CompositerError::MkDirLib(ref why) |
            CompositerError::ReadDirGit(ref why) |
            CompositerError::ReadDirLib(ref why) |
            CompositerError::OpenDirLib(ref why) |
            CompositerError::BuildCommand(ref why) |
            CompositerError::ReadManifest(ref why) => Some(why),
            CompositerError::InstallClone(ref why) |
            CompositerError::UpdateRepOpen(ref why) |
            CompositerError::UpdateRepOrigin(ref why) |
            CompositerError::UpdateRepFetch(ref why) |
            CompositerError::UpdateRepBranch(ref why) |
            CompositerError::UpdateRepObject(ref why) |
            CompositerError::UpdateRepReset(ref why) => Some(why),
            CompositerError::Io(ref why) => Some(why),
            CompositerError::Mount(ref why) => Some(why),
            _ => None,
        }
    }
}

impl From<env::VarError> for CompositerError {
    fn from(_: env::VarError) -> CompositerError {
        CompositerError::NekoPath
    }
}

impl From<io::Error> for CompositerError {
    fn from(_: io::Error) -> CompositerError {
        CompositerError::NekoPath
    }
}

impl PartialEq for CompositerError {
    fn eq(&self, other: &CompositerError) -> bool {
        match (self, other) {
            (&CompositerError::MvFail(_), &CompositerError::MvFail(_)) => true,
            (&CompositerError::RmFile(_), &CompositerError::RmFile(_)) => true,
            (&CompositerError::RmDir(_), &CompositerError::RmDir(_)) => true,
            (&CompositerError::MkDirGit(_), &CompositerError::MkDirGit(_)) => true,
            (&CompositerError::MkDirLib(_), &CompositerError::MkDirLib(_)) => true,
            (&CompositerError::ReadDirGit(_), &CompositerError::ReadDirGit(_)) => true,
            (&CompositerError::ReadDirLib(_), &CompositerError::ReadDirLib(_)) => true,
            (&CompositerError::OpenDirLib(_), &CompositerError::OpenDirLib(_)) => true,
            (&CompositerError::BuildCommand(_), &CompositerError::BuildCommand(_)) => true,
            (&CompositerError::ReadManifest(_), &CompositerError::ReadManifest(_)) => true,
            (&CompositerError::InstallClone(_), &CompositerError::InstallClone(_)) => true,
            (&CompositerError::UpdateRepOpen(_), &CompositerError::UpdateRepOpen(_)) => true,
            (&CompositerError::UpdateRepOrigin(_), &CompositerError::UpdateRepOrigin(_)) => true,
            (&CompositerError::UpdateRepFetch(_), &CompositerError::UpdateRepFetch(_)) => true,
            (&CompositerError::UpdateRepBranch(_), &CompositerError::UpdateRepBranch(_)) => true,
            (&CompositerError::UpdateRepObject(_), &CompositerError::UpdateRepObject(_)) => true,
            (&CompositerError::UpdateRepReset(_), &CompositerError::UpdateRepReset(_)) => true,
            (&CompositerError::Mount(_), &CompositerError::Mount(_)) => true,
            (&CompositerError::BuildExit(_), &CompositerError::BuildExit(_)) => true,
            (&CompositerError::UpdateRepBranchId, &CompositerError::UpdateRepBranchId) => true,
            (&CompositerError::NekoPath, &CompositerError::NekoPath) => true,
            (&CompositerError::UnmountPosition, &CompositerError::UnmountPosition) => true,
            (&CompositerError::UnmountRemove, &CompositerError::UnmountRemove) => true,
            (&CompositerError::ParseManifest, &CompositerError::ParseManifest) => true,
            (&CompositerError::ParseInteger, &CompositerError::ParseInteger) => true,
            (&CompositerError::InstallFormat, &CompositerError::InstallFormat) => true,
            (&CompositerError::InstallExists, &CompositerError::InstallExists) => true,
            _ => false,
        }
    }
}
