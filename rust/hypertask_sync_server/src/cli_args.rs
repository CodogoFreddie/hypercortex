use clap::Clap;
use std::path::PathBuf;

unsafe impl std::marker::Sync for CliArgs {}
unsafe impl std::marker::Send for CliArgs {}

/// Syncing server to replicate hypertask tasks with clients over HTTP
#[derive(Clap, Clone)]
pub struct CliArgs {
    /// Directory containing tasks
    #[clap(long, env = "HYPERTASK_DATA_DIR")]
    pub data_dir: PathBuf,

    /// Should the server daemonise
    #[clap(short, long)]
    pub daemonize: bool,

    /// The hostname that the server will listen under
    #[clap(short, long, env = "HYPERTASK_SERVER_PORT")]
    pub hostname: Option<String>,

    /// The port that the server will listen with
    #[clap(short, long, env = "HYPERTASK_SERVER_PORT")]
    pub port: Option<u16>,

    /// The authorisation secret that must be passed by the client.
    /// The server will generate one if you do not specify
    #[clap(short, long, env = "HYPERTASK_SYNC_SECRET", hide_env_values = true)]
    pub sync_secret: Option<String>,

    /// File to divert stdout to
    #[clap(long)]
    pub std_out_file: Option<PathBuf>,

    /// File to divert stderr to
    #[clap(long)]
    pub std_err_file: Option<PathBuf>,

    /// File to store PID in
    #[clap(long)]
    pub pid_file: Option<PathBuf>,
}

impl hypertask_task_io_operations::ProvidesDataDir for CliArgs {
    fn get_task_state_dir(&self) -> &std::path::PathBuf {
        &self.data_dir
    }
}
