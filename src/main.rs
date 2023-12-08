use clap::Parser;
use std::{
    path::PathBuf,
    fs, fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

#[derive(Parser, Debug)]
#[command(author = "l0rem1psum", version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output_dir: String,
}
fn main() {
    let args = Args::parse();

    let buf_reader = BufReader::new(File::open(&args.input).unwrap());
    let mut buf_writer  = None::<BufWriter<File>>;

    let mut continuous_zeros = 0;
    let mut nalu_index = 0;

    let mut output_path = PathBuf::from(&args.output_dir);
    fs::create_dir_all(&output_path).expect("Failed to create output directory");

    for byte in buf_reader.bytes() {
        let b = byte.unwrap();

        assert!(continuous_zeros != 4, "Encountered invalid H.264 file");

        if b == 0x00 {
            continuous_zeros += 1;
            continue;
        }

        if b == 0x01 && (continuous_zeros == 2 || continuous_zeros == 3) {
            if let Some(ref mut writer) = buf_writer {
                writer.flush().unwrap();
            }

            output_path.push(format!("part-{}.raw", nalu_index));
            buf_writer = Some(BufWriter::new(File::create(&output_path).unwrap()));
            output_path.pop();

            nalu_index += 1;
        }

        match buf_writer {
            Some(ref mut writer) => {
                for _ in 0..continuous_zeros {
                    writer.write(&[0x00]).unwrap();
                }
                continuous_zeros = 0;
                writer.write(&[b]).unwrap();
            }
            None => panic!("Encountered invalid H.264 file"),
        }
    }

    if let Some(ref mut writer) = buf_writer {
        writer.flush().unwrap();
    }
}
