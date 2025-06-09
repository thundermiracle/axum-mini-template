pub mod controllers;
pub mod requests;
pub mod presenters;

use axum::{Router, routing::get};
use std::sync::Arc;
use crate::DI::Container;

pub use controllers::{GetProductsController, GetProductController, BuyProductController};
pub use requests::BuyProductRequest;
pub use presenters::ProductPresenter;

/// Products モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetProductsController::routes())
        .merge(GetProductController::routes())
        .merge(BuyProductController::routes())
} 