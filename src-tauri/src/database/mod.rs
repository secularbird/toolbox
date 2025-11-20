mod init;
mod operations;
mod evidence_operations;

pub use init::init_database;
pub use operations::{
    add_reminder,
    get_all_reminders,
    toggle_reminder,
    delete_reminder,
};
pub use evidence_operations::{
    add_evidence,
    get_evidence_by_id,
    get_evidence_by_reminder,
    get_all_evidence,
    update_evidence_description,
    delete_evidence,
};
