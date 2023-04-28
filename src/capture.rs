use raspicam::{Camera, ImageType};
use std::fs::File;
use std::io::Write;

fn capture_video() {
    let mut camera = Camera::new().unwrap();
    camera
        .imv_video(&mut |data| {
            let mut file = File::create("output.h264").unwrap();
            file.write_all(data).unwrap();
        })
        .unwrap();
}

