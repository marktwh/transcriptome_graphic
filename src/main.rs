use std::error::Error;
use std::str::FromStr;
use image::{ImageBuffer, Luma};
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("lengths.csv")?;

    let mut lengths = Vec::new();
    for result in rdr.records() {
        let record = result?;

        // Try to parse the length in the third column, and skip the row if it's not a valid number
        match u32::from_str(&record[2]) {
            Ok(length) => lengths.push(length),
            _ => continue,
        };
    }

    let count = lengths.len();
    let max_length = match lengths.iter().max() {
        Some(max) => *max,
        None => 0, // or any other value that makes sense in your context
    };

    // Sort in descending order
    lengths.sort();
    lengths.reverse();

    let width = max_length;
    let max_length_f32 = max_length as f32;
    let height = count as u32 * 2;

    // Create a new ImageBuffer with a specific width and height, filled with white pixels
    let mut img = ImageBuffer::from_pixel(width, height, Luma([255u8]));

    // Draw lines on the image according to the lengths
    for (i, &length) in lengths.iter().enumerate() {
        let start_x = ((max_length_f32 - length as f32) / 2.0).round() as u32;
        let end_x = u32::min(start_x + length, width - 1);
        let y = i as u32 * 2;
    
        for x in start_x..=end_x {
            img.put_pixel(x, y, Luma([0u8])); // Black color
        }
    }

    // Save the image as a .png file
    img.save("output.png")?;

    Ok(())
}