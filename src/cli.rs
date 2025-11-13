use clap::Parser;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
use jemallocator::Jemalloc;

#[cfg(not(all(target_env = "musl")))]
use mimalloc::MiMalloc;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(not(all(target_env = "musl")))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser)]
#[command(about = "The Doge Project Server for API service")]
struct Cli {
    #[arg(short = 'p', long, name = "PORT", help = "Listen port")]
    port: Option<u16>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    {
        let cli = Cli::parse();
        let _ = ld_::interface(
            cli.port,
        );
    }

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
