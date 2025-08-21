use std::fs;
use std::path::Path;
use std::io::Write;
use std::collections::BTreeMap;
use exr::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let data_folder = "data";
    
    if !Path::new(data_folder).exists() {
        eprintln!("Data folder '{}' does not exist", data_folder);
        return Ok(());
    }
    
    let entries = fs::read_dir(data_folder)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("exr") {
            println!("Processing: {}", path.display());
            
            match process_exr_file(&path) {
                Ok(()) => println!("Successfully processed: {}", path.display()),
                Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
            }
        }
    }
    
    Ok(())
}

fn process_exr_file(exr_path: &Path) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let image = read_all_data_from_file(exr_path)?;
    
    let file_stem = exr_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name")?;
    
    let output_path = format!("{}.txt", file_stem);
    let mut output_file = fs::File::create(&output_path)?;
    
    writeln!(output_file, "EXR File Analysis: {}", exr_path.display())?;
    writeln!(output_file, "==========================================")?;
    writeln!(output_file)?;
    
    writeln!(output_file, "Image Attributes:")?;
    writeln!(output_file, "  Display Window: {:?}", image.attributes.display_window)?;
    writeln!(output_file, "  Pixel Aspect Ratio: {}", image.attributes.pixel_aspect)?;
    if let Some(chromaticities) = &image.attributes.chromaticities {
        writeln!(output_file, "  Chromaticities: {:?}", chromaticities)?;
    }
    if let Some(time_code) = &image.attributes.time_code {
        writeln!(output_file, "  Time Code: {:?}", time_code)?;
    }
    writeln!(output_file)?;
    
    writeln!(output_file, "Custom Attributes:")?;
    for (name, value) in &image.attributes.other {
        writeln!(output_file, "  {}: {:?}", name, value)?;
    }
    writeln!(output_file)?;
    
    for (layer_index, layer) in image.layer_data.iter().enumerate() {
        writeln!(output_file, "Layer {} Information:", layer_index + 1)?;
        writeln!(output_file, "  Layer Name: {:?}", layer.attributes.layer_name)?;
        writeln!(output_file, "  Size: {}x{}", layer.size.width(), layer.size.height())?;
        writeln!(output_file, "  Encoding: {:?}", layer.encoding)?;
        writeln!(output_file, "  Compression: {:?}", layer.encoding.compression)?;
        writeln!(output_file, "  Line Order: {:?}", layer.encoding.line_order)?;
        writeln!(output_file)?;
        
        writeln!(output_file, "  Layer Attributes:")?;
        for (attr_name, attr_value) in &layer.attributes.other {
            writeln!(output_file, "    {}: {:?}", attr_name, attr_value)?;
        }
        writeln!(output_file)?;
        
        writeln!(output_file, "  Channel Groups:")?;
        
        let mut channel_groups: BTreeMap<String, Vec<&_>> = BTreeMap::new();
        
        for channel in &layer.channel_data.list {
            let channel_name = channel.name.to_string();
            let group_name = if let Some(dot_pos) = channel_name.find('.') {
                channel_name[..dot_pos].to_string()
            } else {
                "Basic".to_string()
            };
            
            channel_groups.entry(group_name).or_insert_with(Vec::new).push(channel);
        }
        
        for (group_name, channels) in channel_groups {
            writeln!(output_file, "    {} Channels:", group_name)?;
            for channel in channels {
                writeln!(output_file, "      {}", channel.name)?;
                writeln!(output_file, "        Sample Data: {:?}", channel.sample_data)?;
                writeln!(output_file, "        Sampling: {:?}", channel.sampling)?;
                writeln!(output_file, "        Quantize Linearly: {}", channel.quantize_linearly)?;
            }
            writeln!(output_file)?;
        }
        writeln!(output_file)?;
    }
    
    Ok(())
}