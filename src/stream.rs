use rocket::response::Stream;
use std::fs::File;

#[get("/video")]
fn video_stream() -> Stream<File> {
    Stream::from(File::open("output.h264").unwrap())
}

