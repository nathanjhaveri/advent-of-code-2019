const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;
const PIXELS_PER_LAYER: usize = LAYER_WIDTH * LAYER_HEIGHT;

#[derive(Eq, PartialEq)]
enum Pixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

fn load_image(image_str: &str) -> Vec<u8> {
    image_str
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unexpected pixel: {}", c),
        })
        .collect()
}

fn load_pixels(image_str: &str) -> Vec<Pixel> {
    image_str
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            _ => panic!("Unexpected pixel: {}", c),
        })
        .collect()
}

pub fn count_image(image_str: &str) -> u32 {
    let image = load_image(image_str);
    let mut counts = Vec::new();
    let layer_count = image.len() / PIXELS_PER_LAYER;

    for _ in 0..layer_count {
        counts.push([0; 3]);
    }

    for (i, &pixel) in image.iter().enumerate() {
        let counts_idx = i / PIXELS_PER_LAYER;
        counts[counts_idx][pixel as usize] += 1;
    }

    let mut min_0s_layer = 0;
    let mut min_0s = counts[min_0s_layer][0];
    for (i, layer) in counts.iter().enumerate() {
        if layer[0] < min_0s {
            min_0s = layer[0];
            min_0s_layer = i;
        }
    }

    counts[min_0s_layer][1] * counts[min_0s_layer][2]
}

pub fn print_image(image: &str) -> String {
    let image = load_pixels(image);
    let mut printed = String::new();

    for j in 0..LAYER_HEIGHT {
        for i in 0..LAYER_WIDTH {
            let mut pixel_index = j * LAYER_WIDTH + i;
            while image[pixel_index] == Pixel::Transparent && pixel_index < image.len() {
                pixel_index += PIXELS_PER_LAYER;
            }

            let printable = match image[pixel_index] {
                Pixel::Transparent => panic!("Can't print transparent pixel at {}", pixel_index),
                Pixel::Black => ' ',
                Pixel::White => '\u{2588}', // Solid box
            };

            printed.push(printable);
        }

        printed.push('\n');
    }

    printed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn eight_1() {
        let img = read_to_string("image.txt").unwrap();
        let count = count_image(&img);
        assert_eq!(count, 1485);
    }

    #[test]
    fn eight_2() {
        let img = read_to_string("image.txt").unwrap();
        let print = print_image(&img);
        let expected = "\
███  █     ██  █  █ ████ 
█  █ █    █  █ █ █  █    
█  █ █    █  █ ██   ███  
███  █    ████ █ █  █    
█ █  █    █  █ █ █  █    
█  █ ████ █  █ █  █ █    \n";
        assert_eq!(print, expected);
    }
}
