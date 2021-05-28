use cargo::core::Workspace;
use cargo::util::command_prelude::ArgMatches;
use cargo::Config;

use crate::build::cbuild;
use crate::install::cinstall;

pub fn cbuild_ws(
    ws: &mut Workspace,
    config: &Config,
    args: &ArgMatches<'_>,
    default_profile: &str,
) -> anyhow::Result<()> {
    for m in ws.members() {
        if m.library().is_some() {
            let mut w = Workspace::new(m.manifest_path(), config)?;
            w.set_target_dir(ws.target_dir());
            let _ = cbuild(&mut w, config, args, default_profile)?;
        }
    }

    Ok(())
}

pub fn cinstall_ws(
    ws: &mut Workspace,
    config: &Config,
    args: &ArgMatches<'_>,
) -> anyhow::Result<()> {
    for m in ws.members() {
        if m.library().is_some() {
            let mut w = Workspace::new(m.manifest_path(), config)?;
            w.set_target_dir(ws.target_dir());

            let (build_targets, install_paths, capi_config, _, _) =
                cbuild(&mut w, config, args, "release")?;

            cinstall(&w, &capi_config, build_targets, install_paths)?;
        }
    }

    Ok(())
}
