
#[derive(Clone)]
pub enum BuilderState {
    Idle,
    Running,
    Abort,
    Done,
}

#[derive(Clone)]
pub struct AppState {
    pub builder: BuilderState,
}



impl Default for AppState {
    fn default() -> Self {
        Self {
            builder: BuilderState::Idle,
        }
    }
}

