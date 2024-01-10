use crate::menu::{admin_menu, librarian_menu, reader_menu};
use rib_backend::state::{Role, State};

pub async fn main_menu(state: &State) {
    match state.role().unwrap() {
        Role::Administrator => admin_menu(state).await,
        Role::Librarian => librarian_menu(state).await,
        Role::Reader => reader_menu(state).await,
    }
}
