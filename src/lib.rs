mod config;
mod fronts;
mod socks2http;
use structopt::StructOpt;

use crate::{config::Opt, connect::ConnectDaemon, debugpack::DebugPack};
mod binderproxy;
mod china;
mod connect;
mod conninfo_store;
mod debugpack;
mod main_bridgetest;
mod metrics;
mod sync;

pub fn dispatch() -> anyhow::Result<()> {
    std::env::remove_var("http_proxy");
    std::env::remove_var("https_proxy");

    let opt = Opt::from_args();
    smolscale::block_on(async move {
        match opt {
            Opt::Connect(opt) => {
                let _daemon = ConnectDaemon::start(opt).await?;
                loop {
                    // let log = recv_logs.recv().await?;
                    // daemon.debug().add_logline(&log);
                }
            }
            Opt::Sync(opt) => sync::main_sync(opt.clone()).await,
            Opt::BinderProxy(opt) => binderproxy::main_binderproxy(opt.clone()).await,
            Opt::BridgeTest(opt) => main_bridgetest::main_bridgetest(opt.clone()).await,
            Opt::DebugPack(opt) => {
                let pack = DebugPack::new(&opt.common.debugpack_path)?;
                pack.backup(&opt.export_to)?;
                Ok(())
            }
        }
    })
}
