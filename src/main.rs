use tiny_skia::{Pixmap, Paint, PathBuilder, Color};
use image::{ImageBuffer, Rgba};

fn main() {
    // --- ① 空のキャンバス（64×64 ピクセル） ---
    let mut canvas = Pixmap::new(64, 64).expect("Cannot create canvas");

    // --- ② 円パスを作成（中心 32,32 半径 20） ---
    let mut pb = PathBuilder::new();
    pb.push_circle(32.0, 32.0, 20.0);
    let circle_path = pb.finish().unwrap();

    // --- ③ 赤い塗りつぶしブラシ ---
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(255, 0, 0, 255)); // (R,G,B,A)

    // --- ④ 描画 ---
    canvas.fill_path(
        &circle_path,
        &paint,
        tiny_skia::FillRule::Winding,
        tiny_skia::Transform::identity(),
        None,
    );

    // --- ⑤ バッファを PNG として保存 ---
    let raw = canvas.data();
    let img: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(64, 64, raw.to_vec()).expect("buffer size mismatch");
    img.save("circle.png").expect("failed to write PNG");

    println!("circle.png を出力しました");
}
