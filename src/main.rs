use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: qrshot <url>");
            std::process::exit(1);
        });

    let code = QrCode::new(url.as_bytes())
        .unwrap_or_else(|e| {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        });

    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    println!("{}", image);
}
