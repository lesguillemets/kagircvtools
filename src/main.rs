use clap::{Parser, Subcommand};
use kagircvtools::video::get_nth_frame;
use kagircvtools::video::save::SpawnSettings;
use std::path::Path;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GetNth {
            file,
            frames,
            out_dir,
        } => {
            let file_path = Path::new(&file);
            let base_name = file_path
                .file_stem()
                .expect("no ext?")
                .to_owned()
                .into_string()
                .unwrap();
            let fs = get_nth_frame::FrameSetting::Frames(frames);
            let g = get_nth_frame::GetNthFrame {
                file,
                frame_setting: fs,
                spawn_setting: SpawnSettings {
                    base_name,
                    dir: out_dir.to_owned(),
                    ext: String::from("png"),
                },
            };
            g.run()
        }
    }
}

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    GetNth {
        #[arg(long)]
        file: String,
        #[arg(long)]
        frames: Vec<usize>,
        #[arg(short, long)]
        out_dir: String,
    },
}
