use cucumber::{cli, runner, World};
use cucumber_rs_bug::world;

#[derive(cli::Args)]
struct CustomOpts {
    /// Feature directory, e.g., `tests/features`
    #[arg(long)]
    feature_dir: String,
}

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let opts = cli::Opts::<_, runner::basic::Cli, _, CustomOpts>::parsed();
    let dir = opts.custom.feature_dir.clone();

    world::World::cucumber()
        .fail_on_skipped()
        .with_cli(opts)
        .run_and_exit(dir)
        .await;
}
