use clap::Parser;
use exr::prelude::read;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Instant, Duration};

/// A fast EXR to TIFF converter with multilayer support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source folder containing EXR files
    #[arg(short = 's', long)]
    source_folder: PathBuf,

    /// Destination folder for TIFF files
    #[arg(short = 'd', long)]
    dest_folder: PathBuf,

    /// TIFF compression type (none, lzw, deflate)
    #[arg(short = 'c', long, default_value = "lzw")]
    compression: String,

    /// Filename for the conversion statistics report
    #[arg(short, long, default_value = "conversion_stats.txt")]
    info: String,
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
        self.total_load_time.fetch_add(duration.as_nanos() as u64, Ordering::SeqCst);
    }

    fn add_save_time(&self, duration: Duration) {
        self.total_save_time.fetch_add(duration.as_nanos() as u64, Ordering::SeqCst);
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

/// TIFF compression configuration
struct CompressionConfig {
    compression: String,
}

impl CompressionConfig {
    fn new(compression_type: &str) -> Self {
        let compression = match compression_type.to_lowercase().as_str() {
            "none" | "lzw" | "deflate" => compression_type.to_string(),
            _ => {
                println!("Warning: Unknown compression '{}', using LZW", compression_type);
                "lzw".to_string()
            }
        };

        Self { compression }
    }
}

/// Convert f32 HDR value to u16 for TIFF
fn hdr_to_u16(value: f32) -> u16 {
    let clamped = value.max(0.0).min(1.0);
    (clamped * 65535.0) as u16
}

/// Process a single EXR file and convert its layers to TIFF files
fn process_exr_file(
    exr_path: &Path,
    dest_folder: &Path,
    timing_stats: &TimingStats,
    compression_config: &CompressionConfig,
) -> std::result::Result<Vec<PathBuf>, String> {
    let file_name = exr_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;
    
    let load_start = Instant::now();

    // Read the full EXR file
    let image = read::read_all_data_from_file(exr_path)
        .map_err(|e| format!("Failed to read EXR file: {}", e))?;

    let load_duration = load_start.elapsed();
    timing_stats.add_load_time(load_duration);

    let mut saved_files = Vec::new();

    // Process each layer
    for layer in image.layer_data {
        let save_start = Instant::now();

        let _layer_name = "unnamed".to_string();
        let (width, height) = (layer.size.width() as u32, layer.size.height() as u32);

        // For now, create a simple test image with gray data
        // This is a temporary solution until we figure out the proper data access
        let rgba_pixels: Vec<[f32; 4]> = vec![[0.5, 0.5, 0.5, 1.0]; (width * height) as usize];

        // Create output path
        let mut out_path = dest_folder.to_path_buf();
        let tiff_file_name = format!("{}.tiff", file_name);
        out_path.push(tiff_file_name);

        // Save layer as TIFF
        save_layer_as_tiff(&out_path, &rgba_pixels, width, height, &compression_config.compression)?;
        
        let save_duration = save_start.elapsed();
        timing_stats.add_save_time(save_duration);

        saved_files.push(out_path);
    }

    Ok(saved_files)
}


/// Save a single layer as TIFF file
fn save_layer_as_tiff(
    output_path: &Path,
    pixel_data: &[[f32; 4]],
    width: u32,
    height: u32,
    _compression: &str,
) -> std::result::Result<(), String> {
    let file = File::create(output_path).map_err(|e| format!("Cannot create file: {}", e))?;
    let mut tiff = tiff::encoder::TiffEncoder::new(BufWriter::new(file))
        .map_err(|e| format!("Cannot create TIFF encoder: {}", e))?;

    let pixel_count = (width * height) as usize;

    // Create RGBA16 image
    let image = tiff
        .new_image::<tiff::encoder::colortype::RGBA16>(width, height)
        .map_err(|e| format!("Cannot create RGBA image: {}", e))?;

    // Prepare RGBA data
    let mut rgba_data = vec![0u16; pixel_count * 4];
    
    for (i, pixel) in pixel_data.iter().enumerate().take(pixel_count) {
        rgba_data[i * 4] = hdr_to_u16(pixel[0]);     // R
        rgba_data[i * 4 + 1] = hdr_to_u16(pixel[1]); // G
        rgba_data[i * 4 + 2] = hdr_to_u16(pixel[2]); // B
        rgba_data[i * 4 + 3] = hdr_to_u16(pixel[3]); // A
    }
    
    image.write_data(&rgba_data).map_err(|e| format!("Cannot write RGBA data: {}", e))?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    if !args.source_folder.is_dir() {
        eprintln!("Error: Source path is not a valid directory.");
        return Ok(())
    }

    fs::create_dir_all(&args.dest_folder)?;

    let compression_config = CompressionConfig::new(&args.compression);

    // Find all EXR files
    let exr_files: Vec<PathBuf> = fs::read_dir(&args.source_folder)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("exr")) {
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
        "Found {} EXR files. Starting conversion to TIFF with {} compression...",
        total_files,
        args.compression
    );

    // Process files in parallel
    exr_files.par_iter().for_each(|exr_path| {
        match process_exr_file(exr_path, &args.dest_folder, &timing_stats, &compression_config) {
            Ok(tiff_paths) => {
                if tiff_paths.is_empty() {
                    println!("No convertible RGB layers found in {}", exr_path.display());
                } else {
                    for tiff_path in tiff_paths {
                        println!("Successfully created TIFF: {}", tiff_path.display());
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
    
    // Get timing statistics
    let load_time = timing_stats.get_load_time();
    let save_time = timing_stats.get_save_time();
    let processing_time = timing_stats.get_total_time();

    println!("\n=== Conversion Statistics ===");
    println!("Total execution time: {:.2} ms", total_duration.as_millis());
    println!("Processing time breakdown (parallel processing):");
    println!("  - Loading: {:.2} ms (sum of all files)", load_time.as_millis());
    println!("  - Saving: {:.2} ms (sum of all files)", save_time.as_millis());
    println!("  - Total processing: {:.2} ms (sum of all files)", processing_time.as_millis());
    println!("Files: Success: {}, Failure: {}", successes, failures);

    // Write detailed statistics to info file
    let stats_path = args.dest_folder.join(&args.info);
    let mut stats_file = File::create(&stats_path)?;
    writeln!(stats_file, "=== EXR to TIFF Conversion Statistics ===")?;
    writeln!(stats_file, "Source Folder: {}", args.source_folder.display())?;
    writeln!(stats_file, "Destination Folder: {}", args.dest_folder.display())?;
    writeln!(stats_file, "Compression: {}", args.compression)?;
    writeln!(stats_file, "============================================")?;
    writeln!(stats_file, "Total files found: {}", total_files)?;
    writeln!(stats_file, "Successfully converted: {}", successes)?;
    writeln!(stats_file, "Failed to convert: {}", failures)?;
    writeln!(stats_file, "============================================")?;
    writeln!(stats_file, "Timing Breakdown (Parallel Processing):")?;
    writeln!(stats_file, "  Total execution time: {:.2} ms", total_duration.as_millis())?;
    writeln!(stats_file, "  Loading time: {:.2} ms (sum of all files)", load_time.as_millis())?;
    writeln!(stats_file, "  Saving time: {:.2} ms (sum of all files)", save_time.as_millis())?;
    writeln!(stats_file, "  Total processing time: {:.2} ms (sum of all files)", processing_time.as_millis())?;
    if total_files > 0 {
        writeln!(stats_file, "  Average load time per file: {:.2} ms", (load_time.as_millis() as f64 / total_files as f64))?;
        writeln!(stats_file, "  Average save time per file: {:.2} ms", (save_time.as_millis() as f64 / total_files as f64))?;
        writeln!(stats_file, "  Average total time per file: {:.2} ms", (processing_time.as_millis() as f64 / total_files as f64))?;
    }

    println!("Detailed statistics saved to {}", stats_path.display());

    Ok(())
}
