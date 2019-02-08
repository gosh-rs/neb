// mods

// [[file:~/Workspace/Programming/gosh/nebrun/neb.note::*mods][mods:1]]
mod neb;

pub use crate::neb::NEB;
// mods:1 ends here

// base

// [[file:~/Workspace/Programming/gosh/nebrun/neb.note::*base][base:1]]
pub(crate) mod common {
    pub use quicli::prelude::*;
    pub type Result<T> = ::std::result::Result<T, Error>;
}
// base:1 ends here
