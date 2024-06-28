#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::debug_handler;

use crate::models::_entities::comments::{ActiveModel};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub content: Option<String>,
    pub article_id: i32,
    }

impl Params {
    fn update(&self, item: &mut ActiveModel) {
      item.content = Set(self.content.clone());
      item.article_id = Set(self.article_id.clone());
    }
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("comments")
        .add("/", post(add))
}
