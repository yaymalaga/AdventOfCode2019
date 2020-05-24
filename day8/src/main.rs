use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "input";

type Image = Vec<u32>;
type RawImage = Vec<Layer>;
type Layer = Vec<Vec<u32>>;

struct ImageDecoder {}

impl ImageDecoder {
    fn load_raw_image(data: &str, wide: usize, tall: usize) -> RawImage {
        let mut raw_image: RawImage = Vec::new();

        let mut layer: Layer = Vec::with_capacity(tall);
        let mut buffer: Vec<u32> = Vec::with_capacity(wide);

        for pixel in data.chars() {
            let digit = pixel.to_digit(10).expect("Invalid digit");

            buffer.push(digit);

            if buffer.len() == buffer.capacity() {
                layer.push(buffer.clone());
                buffer.clear();
            }

            if layer.len() == layer.capacity() {
                raw_image.push(layer.clone());
                layer.clear();
            }
        }

        raw_image
    }

    fn decode_image(raw_image: RawImage, wide: usize, tall: usize) -> Image {
        let mut image: Image = Vec::new();

        let layers_number = raw_image.len();
        let mut buffer: Vec<u32> = Vec::with_capacity(layers_number);
        for row in 0..tall {
            for column in 0..wide {
                for layer in raw_image.iter() {
                    let pixel = layer[row][column];
                    buffer.push(pixel)
                }

                let pixel_image = *buffer.iter().find(|x| **x != 2).unwrap();
                image.push(pixel_image);

                buffer.clear();
            }
        }

        image
    }
}

fn main() {
    let mut file = File::open(format!("src/{}", FILE_NAME)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Invalid data");

    let raw_image: RawImage = ImageDecoder::load_raw_image(&data, 25, 6);

    let mut fewer_0_layer = None;
    let mut layer_counter = None;

    for (layer_index, layer_data) in raw_image.iter().enumerate() {
        let counter = layer_data
            .iter()
            .flatten()
            .fold(0, |i, &pixel| if pixel == 0 { i + 1 } else { i });

        let update = match layer_counter {
            Some(x) => counter < x,
            None => true,
        };

        if update {
            layer_counter = Some(counter);
            fewer_0_layer = Some(layer_index);
        }
    }

    let layer_fewest_0: &Layer = raw_image
        .get(fewer_0_layer.expect("No layer found"))
        .expect("Invalid layer");

    let (counter_1, counter_2) =
        layer_fewest_0
            .iter()
            .flatten()
            .fold((0, 0), |(i_1, i_2), &pixel| {
                if pixel == 1 {
                    (i_1 + 1, i_2)
                } else if pixel == 2 {
                    (i_1, i_2 + 1)
                } else {
                    (i_1, i_2)
                }
            });

    println!("PART 1: {}", counter_1 * counter_2);

    let encoded_image = ImageDecoder::decode_image(raw_image, 25, 6);

    let result = encoded_image
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if (i + 1) % 25 == 0 {
                format!("{}\n", x.to_string())
            } else {
                x.to_string()
            }
        })
        .collect::<String>();

    println!("PART 2: \n{}", result.replace("0", " ").replace("1", "*"));
}
