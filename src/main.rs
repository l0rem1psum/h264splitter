use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

fn main() {
    let buf_reader = BufReader::new(File::open("videos/video1.h264").unwrap());
    let mut buf_writer  = None::<BufWriter<File>>;

    let mut continuous_zeros = 0;
    let mut nalu_index = 0;

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

            buf_writer = Some(BufWriter::new(
                File::create(format!("temp/video1.h264.{}", nalu_index)).unwrap(),
            ));
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
