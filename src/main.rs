
use clap::Parser;
use exr::prelude::{read_all_data_from_file, FlatSamples, Levels};
use image::{ImageBuffer, Rgba, Rgb, Luma};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// A fast EXR to PNG converter with multilayer support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source folder containing EXR files
    #[arg(short = 's', long)]
    source_folder: PathBuf,

    /// Destination folder for PNG files
    #[arg(short = 'd', long)]
    dest_folder: PathBuf,

    /// Filename for the conversion statistics report
    #[arg(short = 't', long, default_value = "conversion_stats.txt")]
    stats: String,
}

/// Statistics for timing operations
struct TimingStats {
    total_load_time: AtomicU64,
    total_save_time: AtomicU64,
}

impl TimingStats {
    fn new() -> Self {
        Self {
            total_load_time: AtomicU64::new(0),
            total_save_time: AtomicU64::new(0),
        }
    }

    fn add_load_time(&self, duration: Duration) {
        self.total_load_time
            .fetch_add(duration.as_nanos() as u64, Ordering::SeqCst);
    }

    fn add_save_time(&self, duration: Duration) {
        self.total_save_time
            .fetch_add(duration.as_nanos() as u64, Ordering::SeqCst);
    }

    fn get_load_time(&self) -> Duration {
        Duration::from_nanos(self.total_load_time.load(Ordering::SeqCst))
    }

    fn get_save_time(&self) -> Duration {
        Duration::from_nanos(self.total_save_time.load(Ordering::SeqCst))
    }

    fn get_total_time(&self) -> Duration {
        self.get_load_time() + self.get_save_time()
    }
}

/// Convert f32 HDR value to u16 for PNG
fn hdr_to_u16(value: f32) -> u16 {
    (value.clamp(0.0, 1.0) * 65535.0) as u16
}

/// Process a single EXR file and convert its layers to PNG files
fn process_exr_file(
    exr_path: &Path,
    dest_folder: &Path,
    timing_stats: &TimingStats,
) -> Result<Vec<PathBuf>, String> {
    let file_name = exr_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    let load_start = Instant::now();

    let image = read_all_data_from_file(exr_path)
        .map_err(|e| format!("Failed to read EXR file: {}", e))?;

    let load_duration = load_start.elapsed();
    timing_stats.add_load_time(load_duration);

    let output_dir = dest_folder.join(file_name);
    fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let mut saved_files = Vec::new();

    for layer in image.layer_data {
        let (width, height) = (
            layer.size.width() as u32,
            layer.size.height() as u32,
        );

        let mut channels_by_layer: HashMap<String, HashMap<String, Vec<f32>>> = HashMap::new();

        for channel in &layer.channel_data.list {
            let channel_name = channel.name.to_string();
            let (layer_name, chan_name) = match channel_name.split_once('.') {
                Some((layer, chan)) => (layer.to_string(), chan.to_string()),
                None => ("default".to_string(), channel_name.clone()),
            };

            let samples = match &channel.sample_data {
                Levels::Singular(samples) => match samples {
                    FlatSamples::F32(samples) => Some(samples.clone()),
                    _ => None,
                },
                _ => None,
            };

            if let Some(samples) = samples {
                channels_by_layer
                    .entry(layer_name)
                    .or_default()
                    .insert(chan_name, samples);
            }
        }

        for (layer_name, channels) in channels_by_layer {
            let save_start = Instant::now();

            let mut png_path = output_dir.clone();
            png_path.push(format!("{}.png", layer_name));

            save_layer_as_png(&png_path, &channels, width, height)?;

            let save_duration = save_start.elapsed();
            timing_stats.add_save_time(save_duration);

            saved_files.push(png_path);
        }
    }

    Ok(saved_files)
}

fn find_channel<'a>(
    channels: &'a HashMap<String, Vec<f32>>,
    candidates: &[&str],
) -> Option<&'a Vec<f32>> {
    for &candidate in candidates {
        if let Some(channel) = channels.get(candidate) {
            return Some(channel);
        }
    }
    None
}

/// Save a single layer as a 16-bit PNG file
fn save_layer_as_png(
    output_path: &Path,
    channels: &HashMap<String, Vec<f32>>,
    width: u32,
    height: u32,
) -> Result<(), String> {
    let r_channel = find_channel(channels, &["R", "r", "red", "Red"]);
    let g_channel = find_channel(channels, &["G", "g", "green", "Green"]);
    let b_channel = find_channel(channels, &["B", "b", "blue", "Blue"]);
    let a_channel = find_channel(channels, &["A", "a", "alpha", "Alpha"]);

    if let (Some(r), Some(g), Some(b)) = (r_channel, g_channel, b_channel) {
        if let Some(a) = a_channel {
            let mut image_buffer: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(width, height);
            for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                let index = (y * width + x) as usize;
                *pixel = Rgba([
                    hdr_to_u16(r[index]),
                    hdr_to_u16(g[index]),
                    hdr_to_u16(b[index]),
                    hdr_to_u16(a[index]),
                ]);
            }
            image_buffer.save(output_path).map_err(|e| format!("Failed to save PNG file: {}", e))?;
        } else {
            let mut image_buffer: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::new(width, height);
            for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                let index = (y * width + x) as usize;
                *pixel = Rgb([
                    hdr_to_u16(r[index]),
                    hdr_to_u16(g[index]),
                    hdr_to_u16(b[index]),
                ]);
            }
            image_buffer.save(output_path).map_err(|e| format!("Failed to save PNG file: {}", e))?;
        }
    } else {
        // Fallback to grayscale for single-channel images or if RGB channels are not found
        if let Some(channel_data) = r_channel.or(g_channel).or(b_channel).or(a_channel).or(channels.values().next()) {
            let mut image_buffer: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::new(width, height);
            for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
                let index = (y * width + x) as usize;
                *pixel = Luma([hdr_to_u16(channel_data[index])]);
            }
            image_buffer.save(output_path).map_err(|e| format!("Failed to save PNG file: {}", e))?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    if !args.source_folder.is_dir() {
        eprintln!("Error: Source path is not a valid directory.");
        return Ok(());
    }

    fs::create_dir_all(&args.dest_folder)?;

    let exr_files: Vec<PathBuf> = fs::read_dir(&args.source_folder)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file()
                    && path.extension().map_or(false, |ext| {
                        ext.eq_ignore_ascii_case("exr")
                    })
                {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    let total_files = exr_files.len();
    let success_count = AtomicUsize::new(0);
    let failure_count = AtomicUsize::new(0);
    let timing_stats = TimingStats::new();

    println!(
        "Found {} EXR files. Starting conversion to PNG...",
        total_files
    );

    exr_files.par_iter().for_each(|exr_path| {
        match process_exr_file(exr_path, &args.dest_folder, &timing_stats) {
            Ok(png_paths) => {
                if png_paths.is_empty() {
                    println!(
                        "No convertible layers found in {}",
                        exr_path.display()
                    );
                } else {
                    for png_path in png_paths {
                        println!("Successfully created PNG: {}", png_path.display());
                    }
                }
                success_count.fetch_add(1, Ordering::SeqCst);
            }
            Err(e) => {
                eprintln!("Failed to process {}: {}", exr_path.display(), e);
                failure_count.fetch_add(1, Ordering::SeqCst);
            }
        }
    });

    let total_duration = start_time.elapsed();
    let successes = success_count.load(Ordering::SeqCst);
    let failures = failure_count.load(Ordering::SeqCst);

    let load_time = timing_stats.get_load_time();
    let save_time = timing_stats.get_save_time();
    let processing_time = timing_stats.get_total_time();

    println!("
=== Conversion Statistics ===");
    println!(
        "Total execution time: {:.2} ms",
        total_duration.as_millis()
    );
    println!("Processing time breakdown (parallel processing):");
    println!(
        "  - Loading: {:.2} ms (sum of all files)",
        load_time.as_millis()
    );
    println!(
        "  - Saving: {:.2} ms (sum of all files)",
        save_time.as_millis()
    );
    println!(
        "  - Total processing: {:.2} ms (sum of all files)",
        processing_time.as_millis()
    );
    println!("Files: Success: {}, Failure: {}", successes, failures);

    let stats_path = args.dest_folder.join(&args.stats);
    let mut stats_file = File::create(&stats_path)?;
    writeln!(stats_file, "=== EXR to PNG Conversion Statistics ===")?;
    writeln!(
        stats_file,
        "Source Folder: {}",
        args.source_folder.display()
    )?;
    writeln!(
        stats_file,
        "Destination Folder: {}",
        args.dest_folder.display()
    )?;
    writeln!(stats_file, "============================================")?;
    writeln!(stats_file, "Total files found: {}", total_files)?;
    writeln!(stats_file, "Successfully converted: {}", successes)?;
    writeln!(stats_file, "Failed to convert: {}", failures)?;
    writeln!(stats_file, "============================================")?;
    writeln!(stats_file, "Timing Breakdown (Parallel Processing):")?;
    writeln!(
        stats_file,
        "  Total execution time: {:.2} ms",
        total_duration.as_millis()
    )?;
    writeln!(
        stats_file,
        "  Loading time: {:.2} ms (sum of all files)",
        load_time.as_millis()
    )?;
    writeln!(
        stats_file,
        "  Saving time: {:.2} ms (sum of all files)",
        save_time.as_millis()
    )?;
    writeln!(
        "  Total processing time: {:.2} ms (sum of all files)",
        processing_time.as_millis()
    )?;
    if total_files > 0 {
        writeln!(
            stats_file,
            "  Average load time per file: {:.2} ms",
            (load_time.as_millis() as f64 / total_files as f64)
        )?;
        writeln!(
            stats_file,
            "  Average save time per file: {:.2} ms",
            (save_time.as_millis() as f64 / total_files as f64)
        )?;
        writeln!(
            stats_file,
            "  Average total time per file: {:.2} ms",
            (processing_time.as_millis() as f64 / total_files as f64)
        )?;
    }

    println!("Detailed statistics saved to {}", stats_path.display());

    Ok(())
}
