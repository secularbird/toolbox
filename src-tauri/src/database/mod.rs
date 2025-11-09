mod init;
mod operations;

pub use init::init_database;
pub use operations::{
    add_reminder,
    get_all_reminders,
    toggle_reminder,
    delete_reminder,
};
