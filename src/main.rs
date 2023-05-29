use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

fn main() {
    let file = File::open("videos/video1.h264").unwrap();

    let buf = BufReader::new(file);
    let mut writer: Option<BufWriter<File>> = None;

    let mut continuous_zeros = 0;
    let mut nal_index = 0;

    for byte in buf.bytes() {
        let b = byte.unwrap();

        assert!(continuous_zeros != 4, "Encountered invalid H.264 file");

        if b == 0x00 {
            continuous_zeros += 1;
            continue;
        }

        if continuous_zeros == 2 || continuous_zeros == 3 {
            if b == 0x01 {
                if let Some(ref mut w) = writer {
                    w.flush().unwrap();
                }

                writer = Some(BufWriter::new(
                    File::create(format!("videos/video1.h264.{}", nal_index)).unwrap(),
                ));
                nal_index += 1;

                for _ in 0..continuous_zeros {
                    writer.as_mut().unwrap().write(&[0x00]).unwrap();
                }
                writer.as_mut().unwrap().write(&[0x01]).unwrap();
                continuous_zeros = 0;
                continue;
            } else {
                for _ in 0..continuous_zeros {
                    writer
                        .as_mut()
                        .expect("Encountered invalid H.264 file")
                        .write(&[0x00])
                        .unwrap();
                }
                continuous_zeros = 0;
            }
        }
        match writer {
            Some(ref mut w) => {
                if continuous_zeros == 1 {
                    w.write(&[0x00]).unwrap();
                    continuous_zeros = 0;
                }
                w.write(&[b]).unwrap();
            }
            None => panic!("no possible"),
        }
    }
    match writer {
        None => return,
        Some(ref mut w) => w.flush().unwrap(),
    }
}
