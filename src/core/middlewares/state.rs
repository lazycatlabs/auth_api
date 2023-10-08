use crate::core::config::di::DiContainer;
use crate::core::types::DBConn;

#[derive(Clone)]
pub struct AppState {
    pub di_container: DiContainer,
}

impl AppState {
    pub fn new(pool: DBConn) -> Self {
        let di_container = DiContainer::new(&pool);
        Self { di_container }
    }
}