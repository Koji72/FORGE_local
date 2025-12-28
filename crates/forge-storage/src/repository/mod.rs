//! Repository implementations for CRUD operations

pub mod tasks;
pub mod notes;
pub mod goals;
pub mod links;
pub mod settings;
pub mod agent_runs;

pub use tasks::{TaskRepository, TaskFilter};
pub use notes::{NoteRepository, NoteFilter};
pub use goals::{GoalRepository, GoalFilter};
pub use links::LinkRepository;
pub use settings::SettingsRepository;
pub use agent_runs::{AgentRunRepository, AgentRunFilter};

use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Database connection wrapper
pub type DbConnection = Arc<Mutex<Connection>>;

/// All repositories bundled together
pub struct Repositories {
    pub tasks: TaskRepository,
    pub notes: NoteRepository,
    pub goals: GoalRepository,
    pub links: LinkRepository,
    pub settings: SettingsRepository,
    pub agent_runs: AgentRunRepository,
}

impl Repositories {
    /// Create all repositories from a connection
    pub fn new(conn: DbConnection) -> Self {
        Self {
            tasks: TaskRepository::new(conn.clone()),
            notes: NoteRepository::new(conn.clone()),
            goals: GoalRepository::new(conn.clone()),
            links: LinkRepository::new(conn.clone()),
            settings: SettingsRepository::new(conn.clone()),
            agent_runs: AgentRunRepository::new(conn),
        }
    }
}
