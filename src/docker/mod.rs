mod custom;
mod engine;
mod local;
pub mod remote;
mod shared;

pub use self::engine::*;
pub use self::shared::*;

use std::path::Path;
use std::process::ExitStatus;

use crate::cargo::CargoMetadata;
use crate::errors::*;
use crate::shell::MessageInfo;
use crate::{Config, Target};

#[allow(clippy::too_many_arguments)] // TODO: refactor
pub fn run(
    engine: &Engine,
    target: &Target,
    args: &[String],
    metadata: &CargoMetadata,
    config: &Config,
    uses_xargo: bool,
    sysroot: &Path,
    msg_info: MessageInfo,
    docker_in_docker: bool,
    cwd: &Path,
) -> Result<ExitStatus> {
    if engine.is_remote {
        remote::run(
            engine,
            target,
            args,
            metadata,
            config,
            uses_xargo,
            sysroot,
            msg_info,
            docker_in_docker,
            cwd,
        )
    } else {
        local::run(
            engine,
            target,
            args,
            metadata,
            config,
            uses_xargo,
            sysroot,
            msg_info,
            docker_in_docker,
            cwd,
        )
    }
}
