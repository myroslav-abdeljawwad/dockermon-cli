use std::{error::Error, io::Write};

use bollard::{
    API_DEFAULT_VERSION,
    container::{ListContainersOptions, StatsOptions},
    Docker,
};
use clap::{Parser, Subcommand};
use futures_util::stream::StreamExt;
use serde_json::json;

/// Created by Myroslav Mokhammad Abdeljawwad
///
/// A CLI tool to stream real-time CPU and memory statistics of running Docker containers.
#[derive(Parser)]
#[clap(author = "Myroslav Mokhammad Abdeljawwad", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Stream stats for a specific container
    Stats { id: String },

    /// Stream stats for all running containers
    All {},
}

async fn stream_container_stats(
    docker: &Docker,
    container_id: &str,
) -> Result<(), Box<dyn Error>> {
    let options = Some(StatsOptions {
        stream: true,
        one_shot: false,
    });

    let mut stats_stream = docker.stats(container_id, options).await?;

    println!("Streaming stats for container {}", container_id);
    while let Some(stats_result) = stats_stream.next().await {
        match stats_result {
            Ok(stats) => {
                // Convert relevant stats to a JSON object
                let cpu_percent = calculate_cpu_percentage(&stats.cpu_stats, &stats.precpu_stats);
                let memory_usage = stats.memory_stats.usage.unwrap_or(0);
                let memory_limit = stats.memory_stats.limit.unwrap_or(1);

                let output = json!({
                    "id": container_id,
                    "cpu_percent": cpu_percent,
                    "memory_usage_bytes": memory_usage,
                    "memory_limit_bytes": memory_limit
                });

                println!("{}", serde_json::to_string_pretty(&output)?);
            }
            Err(e) => eprintln!("Error streaming stats for {}: {}", container_id, e),
        }
    }

    Ok(())
}

fn calculate_cpu_percentage(
    cpu_stats: &bollard::container::CpuStats,
    precpu_stats: &bollard::container::PreCpuStats,
) -> f64 {
    let cpu_delta = cpu_stats.cpu_usage.total_usage - precpu_stats.cpu_usage.total_usage;
    let system_delta = cpu_stats.system_cpu_usage.unwrap_or(0)
        - precpu_stats.system_cpu_usage.unwrap_or(0);
    if system_delta > 0 && cpu_delta > 0 {
        (cpu_delta as f64 / system_delta as f64) * cpu_stats.online_cpus as f64 * 100.0
    } else {
        0.0
    }
}

async fn stream_all_container_stats(docker: &Docker) -> Result<(), Box<dyn Error>> {
    let options = Some(ListContainersOptions::<String> {
        all: false,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await?;
    if containers.is_empty() {
        println!("No running containers found.");
        return Ok(());
    }

    for container in containers {
        if let Some(id) = container.id {
            // Spawn a task per container to stream stats concurrently
            let docker_clone = docker.clone();
            tokio::spawn(async move {
                if let Err(e) = stream_container_stats(&docker_clone, &id).await {
                    eprintln!("Failed to stream stats for {}: {}", id, e);
                }
            });
        }
    }

    // Keep the main task alive while child tasks run
    futures_util::future::pending::<()>().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Initialize Docker connection
    let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");

    match &cli.command {
        Commands::Stats { id } => stream_container_stats(&docker, id).await,
        Commands::All {} => stream_all_container_stats(&docker).await,
    }
}