use crate::core::{
    config::di::DiContainer,
    types::DBConn,
};

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