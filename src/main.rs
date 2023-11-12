use futures::prelude::*;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::Api,
    runtime::{watcher, WatchStreamExt},
    Client,
};
use tracing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;
    let api = Api::<Pod>::default_namespaced(client);
    let watcher_config = watcher::Config::default();

    watcher(api, watcher_config)
        .applied_objects()
        .default_backoff()
        .try_for_each(|p| async move {
            info!("{:?}", p);
            Ok(())
        })
        .await?;
    Ok(())
}
