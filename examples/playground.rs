use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use myloco::{app::App, models::_entities::articles};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let ctx = playground::<App>().await.context("playground")?;

    let active_model: articles::ActiveModel = articles::ActiveModel {
        title: Set(Some("how to build apps in 3 steps".to_string())),
        content: Set(Some("use Loco: https://loco.rs".to_string())),
        ..Default::default()
    };

    active_model.insert(&ctx.db).await.unwrap();
    let res = articles::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);

    Ok(())
}
