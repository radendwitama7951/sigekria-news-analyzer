use std::sync::{Arc, OnceLock};

use warp::Filter;

use super::models::UserIdT;

#[derive(Debug, Default, Clone)]
pub struct AppConfigT {
    pub local_backend_api: String,
    pub local_default_user: UserIdT,

    pub remote_backend_api: String,
    pub remote_default_user: UserIdT,
}

pub type AppConfig = Arc<AppConfigT>;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn load_config() -> AppConfig {
    CONFIG
        .get_or_init(|| {
            Arc::new(AppConfigT {
                remote_backend_api: "http://localhost:8000/api/v0".into(), // BEING SWITCHED
                local_default_user: "fa160d0b-2922-496e-a7b0-abc133c48ca7".into(),
                // local_backend_api: "https://main-bvxea6i-k67tm7kfclutw.au.platformsh.site/api/v0"
                //     .into(),
                local_backend_api: "http://localhost:8000/api/v0".into(),
                remote_default_user: "remoteuser".into(),
            })
        })
        .clone()
}

pub fn with_app_config(
    config: AppConfig,
) -> impl Filter<Extract = (AppConfig,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}
