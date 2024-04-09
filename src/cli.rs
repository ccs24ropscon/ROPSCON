use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "ROPSCON")]
#[command(author = "Xuyang Liu <liuxuyangcs@gmail.com>")]
#[command(about = "ROPSCON", version)]
pub(crate) struct Cli {
    /// Path to the config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    /// Set ID of current node.
    #[arg(short, long, default_value_t = 0)]
    pub(crate) id: u64,

    /// Addresses of known nodes.
    #[arg(short, long, default_value_t = String::from("localhost:8123"))]
    addr: String,

    //#[arg(short, long)]
    //pub(crate) disable_ropscon: bool,

    /// Set injection_rate, default value is 100_000 (tx/s).
    #[arg(short, long)]
    pub(crate) rate: Option<u64>,

    /// Set transaction_size, default value is 256 (Bytes).
    #[arg(short, long)]
    pub(crate) transaction_size: Option<usize>,

    /// Set batch_size, default value is 100 (tx/block).
    #[arg(short, long)]
    pub(crate) batch_size: Option<usize>,

    /// Pretend to be a crash nodes.
    #[arg(long)]
    pub(crate) pretend_crash: bool,

    /// Pretend to be a malicious nodes (always send malicious shares to interfere with consensus).
    #[arg(long)]
    pub(crate) pretend_malicious: bool,

    /// Disable metrics.
    #[arg(long)]
    pub(crate) disable_metrics: bool,

    /// Rotate leadership every `rotate_every` key blocks.
    #[arg(long)]
    pub(crate) leader_rotation: Option<usize>,

    /// Maximum number of transactions in the mempool.
    #[arg(long)]
    pub(crate) mempool_size: Option<usize>,

    /// Pacemaker timeout.
    #[arg(long)]
    pub(crate) timeout: Option<usize>,

    /// Export metrics to file when node exits.
    #[arg(short, long)]
    pub(crate) export_path: Option<PathBuf>,

    /// Subcommands
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub(crate) enum Commands {
    /// Run the node within memory network.
    MemoryTest {
        /// Number of nodes.
        #[arg(short, long, default_value_t = 4)]
        number: usize,
    },

    /// Run the node in memory network with some nodes are crash.
    CrashTest {
        /// Number of failures.
        #[arg(short, long, default_value_t = 1)]
        number: usize,
    },

    /// Run the node in memory network with some nodes are malicious.
    MaliciousTest {
          /// Number of failures.
          #[arg(short, long, default_value_t = 1)]
          number: usize,
    },

    /// Generate all configs for a single test.
    ///
    /// This command distributes `number` replicas into `hosts`,
    /// automatically assign id and addr.
    ConfigGen {
        /// Number of nodes.
        #[arg(short, long, default_value_t = 4)]
        number: usize,

        /// hosts to distribute replicas.
        hosts: Vec<String>,

        /// Path to export configs.
        #[arg(short, long, value_name = "DIR")]
        export_dir: Option<PathBuf>,

        /// automatically export configs to a new directory.
        #[arg(short, long)]
        auto_naming: bool,

        /// Write all the configs and scripts to file.
        #[arg(short, long)]
        write_file: bool,

        /// Number of crash nodes
        #[arg(short, long, default_value_t = 0)]
        crash_nodes: usize,

        /// Number of malicious nodes ((always send malicious shares to interfere with consensus).)
        /// The sum of these two values must follow the rule of BFT,
        /// that 3f+1 <= n.
        #[arg(short, long, default_value_t = 0)]
        malicious_nodes: usize,
    },
}
