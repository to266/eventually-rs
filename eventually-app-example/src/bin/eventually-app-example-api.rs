#![allow(missing_docs)]

use envconfig::Envconfig;

use eventually_app_example::config::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = Config::init()?;
    eventually_app_example::run(config).await
}
