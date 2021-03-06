use std::path::PathBuf;

pub fn collage(images: Vec<PathBuf>, albums: Vec<super::Album>, args: super::Args) {
    use image::GenericImage;

    let x = args.x;
    let y = args.y;

    let mut img = image::DynamicImage::new_rgba8(300 * x, 300 * y);

    for ((i, image), album) in (0..(x * y)).zip(images.iter()).zip(albums.iter()) {
        let img2 = match image::open(image) {
            Ok(im) => im,
            Err(err) => {
                eprintln!("error opening image for {}\n{}", album, err.to_string());
                image::DynamicImage::new_rgba8(300 * x, 300 * y)
            },
        };
        img2.resize_exact(300, 300, image::imageops::CatmullRom);
        let mut img2 = img2.to_rgba();
        if args.captions {
            draw_description(&mut img2, album.artist(), album.title());
        }
        if let Err(x) = img.copy_from(&img2, 300 * (i % x), 300 * (i / x)) {
            eprintln!("{} during copying a picture of {}", &x, &album);
        }
    }
    img.save(format!("{}test.png", args.path_out.display()))
        .unwrap();
}

fn draw_description(
    img: &mut image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    author: &str,
    title: &str,
) {
    use rusttype::{FontCollection, Scale};

    let font = Vec::from(include_bytes!("../data/berlin-email.berlin-email.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    let font_shadow =
        Vec::from(include_bytes!("../data/berlin-email.berlin-email-schaddow.ttf") as &[u8]);
    let font_shadow = FontCollection::from_bytes(font_shadow)
        .unwrap()
        .into_font()
        .unwrap();

    let height = 25.0;
    let scale = Scale {
        x: height,
        y: height,
    };
    let mut draw = |fnt, txt, x, y, col| {
        imageproc::drawing::draw_text_mut(
            img,
            image::Rgba([col, col, col, col]),
            x,
            y,
            scale,
            fnt,
            txt,
        )
    };
    let mut with_shadow = |txt, y| {
        draw(&font, txt, 0, y, 0u8);
        draw(&font_shadow, txt, 0, y, 255u8);
    };

    with_shadow(author, 0);
    with_shadow(title, 25);
}
