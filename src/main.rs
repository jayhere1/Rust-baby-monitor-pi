use rocket::{get, routes};

fn main() {
    std::thread::spawn(move || {
        capture_video();
    });

    // rocket::ignite()
    //     .mount("/", routes![index, login, video_stream])
    //     .attach(Template::fairing())
    //     .launch();
}

