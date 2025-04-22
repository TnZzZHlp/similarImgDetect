use clap::Parser;
use image_hasher::HasherConfig;
use indicatif::{ProgressState, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::{fmt, fs::read_dir, path::Path, sync::Mutex, time::Duration};

mod dsu;
use dsu::DisjointSet;

#[derive(Parser)]
struct Args {
    /// The path to the folder containing the images
    #[clap(short, long)]
    path: String,

    /// The similarity threshold for image comparison (0-1)
    #[clap(short, long, default_value = "0.9")]
    similarity: f32,

    /// Actions after finding similar images
    /// move: Move similar images to a folder
    /// copy: Copy similar images to a folder
    /// delete: Delete similar images except the largest one
    /// print: Print the paths of similar images
    #[clap(short, long, default_value = "print")]
    action: String,

    /// The path to the folder to move/copy similar images to
    #[clap(short, long)]
    target_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let image_paths = find_all_img_recusive(&args.path)?;
    let image_hashes: Vec<Image> = generate_image_hash(image_paths)?; // image_hashes 是 Vec<Image>

    let similar_groups = find_similar_image_groups(image_hashes, args.similarity);

    if similar_groups.is_empty() {
        println!("No similar images found.");
    } else {
        for (i, group) in similar_groups.iter().enumerate() {
            match args.action.as_str() {
                "move" => {
                    if let Some(target_path) = &args.target_path {
                        // Create the target directory if it doesn't exist
                        std::fs::create_dir_all(format!("{}/{}", target_path, i + 1))?;
                        for image in group {
                            let target = format!(
                                "{}/{}/{}",
                                target_path,
                                i + 1,
                                Path::new(image).file_name().unwrap().to_string_lossy()
                            );
                            std::fs::rename(image, target)?;
                        }
                    } else {
                        eprintln!("Target path is required for move action.");
                    }
                }
                "copy" => {
                    if let Some(target_path) = &args.target_path {
                        // Create the target directory if it doesn't exist
                        std::fs::create_dir_all(format!("{}/{}", target_path, i + 1))?;
                        for image in group {
                            let target = format!(
                                "{}/{}/{}",
                                target_path,
                                i + 1,
                                Path::new(image).file_name().unwrap().to_string_lossy()
                            );
                            std::fs::copy(image, target)?;
                        }
                    } else {
                        eprintln!("Target path is required for copy action.");
                    }
                }
                "delete" => {
                    // Find the largest image in the group
                    let largest_image = group
                        .iter()
                        .max_by_key(|img| std::fs::metadata(img).map(|m| m.len()).unwrap_or(0));
                    for image in group {
                        if Some(image) != largest_image {
                            println!("Deleting image: {}", image);
                            std::fs::remove_file(image)?;
                        }
                    }
                }
                "print" => {
                    println!("Group {}: ", i + 1);
                    for image in group {
                        println!("  {}", image);
                    }
                }
                _ => eprintln!("Unknown action: {}", args.action),
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "avif"))]
const IMAGE_FORMATS: [&str; 8] = ["jpg", "jpeg", "png", "bmp", "webp", "tiff", "tif", "ico"];
#[cfg(feature = "avif")]
const IMAGE_FORMATS: [&str; 9] = [
    "jpg", "jpeg", "png", "bmp", "webp", "tiff", "tif", "ico", "avif",
];
fn find_all_img_recusive<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let mut images = Vec::new();
    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                images.extend(find_all_img_recusive(path)?);
            } else if IMAGE_FORMATS.iter().any(|&ext| {
                path.extension()
                    .is_some_and(|e| e.to_ascii_lowercase() == ext)
            }) {
                if let Some(path_str) = path.to_str() {
                    images.push(path_str.to_string());
                }
            }
        }
    }

    Ok(images)
}

struct Image {
    image_path: String,
    hash: image_hasher::ImageHash,
}
fn generate_image_hash(images: Vec<String>) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
    let image_hashes = Mutex::new(Vec::new());
    let hasher = HasherConfig::new().to_hasher();
    let pb = get_progress_bar(images.len() as u64);
    pb.set_message("Calculating image hash");

    images.into_par_iter().for_each(|image| {
        if let Ok(img) = image::open(&image) {
            let hash = hasher.hash_image(&img);
            let mut image_hashes = image_hashes.lock().unwrap();
            image_hashes.push(Image {
                image_path: image,
                hash,
            });
        } else {
            eprintln!("Failed to open image: {}", image);
        }
        pb.inc(1);
    });

    pb.finish_with_message("Calculation image hash complete");
    Ok(image_hashes.into_inner().unwrap())
}

fn find_similar_image_groups(images: Vec<Image>, similarity: f32) -> Vec<Vec<String>> {
    let n = images.len();
    if n < 2 {
        return Vec::new();
    }

    // Compute all similar pairs (i, j)
    let similar_pairs: Vec<(usize, usize)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..n)
                .filter_map(|j| {
                    let dist = images[i].hash.dist(&images[j].hash);
                    // Similarity calculation (assuming a maximum distance of 64, where smaller distances indicate greater similarity)
                    let current_similarity = 1.0 - (dist as f32 / 64.0);

                    if current_similarity >= similarity {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    // Use DSU to merge the sets to which similar pairs belong
    let mut dsu = DisjointSet::new(n);
    for (i, j) in similar_pairs {
        dsu.union(i, j);
    }

    // Group image paths by root node
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (i, image) in images.into_iter().enumerate() {
        let root = dsu.find_root(i);
        groups.entry(root).or_default().push(image.image_path);
    }

    // Collect all groups, optionally filtering out groups with only one element
    groups
        .into_values()
        .filter(|group| group.len() > 1)
        .collect()
}

fn get_progress_bar(len: u64) -> indicatif::ProgressBar {
    let pb = indicatif::ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}
