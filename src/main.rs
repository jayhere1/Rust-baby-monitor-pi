// use rocket::{routes};
// // use capture::capture_video;
// mod capture;
// mod login;
// mod stream;

// fn main() {
//     std::thread::spawn(move || {
//         capture_video();
//     });

//     rocket::ignite()
//         .mount("/", routes![index, login, video_stream])
//         .attach(Template::fairing())
//         .launch();
// }


use std::io::{Read, Write};
use std::net::{TcpListener};

use image::{self, ImageBuffer, Pixel, Rgb};
use libcamera_rs::{Camera, Stream};

fn main() {
    // Create a camera object.
    let mut camera = Camera::new().unwrap();

    // Initialize the camera.
    camera.init().unwrap();

    // Create a stream object with default configuration.
    let mut stream = Stream::new().unwrap();

    // Configure the stream to capture video frames.
    stream.set_format(libcamera_rs::PixelFormat::YUYV, 640, 480).unwrap();

    // Start the capture loop.
    loop {
        // Start the stream capture.
        stream.start(&camera).unwrap();

        // Wait for a frame to be available.
        let buffer = stream.wait().unwrap();

        // Convert the frame data to an image buffer.
        let image = ImageBuffer::<Rgb<u8>, _>::from_raw(
            buffer.width() as u32,
            buffer.height() as u32,
            buffer.data(),
        )
        .unwrap();

        // Release the buffer back to the stream.
        buffer.release().unwrap();

        // Encode the image buffer as a JPEG image.
        let mut jpeg_data = Vec::new();
        image::jpeg::encode(&mut jpeg_data, &image, image::ColorType::Rgb8).unwrap();

        // Send the compressed image over a TCP socket.
        send_frame(jpeg_data);
    }
}

fn send_frame(data: Vec<u8>) {
    // Create a TCP listener on port 8080.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Wait for a client to connect.
    let (mut socket, _) = listener.accept().unwrap();

    // Send the compressed image data over the socket.
    socket.write_all(&data).unwrap();

    // Wait for the client to acknowledge receipt of the data.
    let mut buf = [0; 1];
    socket.read_exact(&mut buf).unwrap();

    // Close the socket.
    drop(socket);
}
