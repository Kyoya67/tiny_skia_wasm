use tiny_skia::{Pixmap, Paint, PathBuilder, Color};
use std::alloc::{alloc, dealloc, Layout};

/// 幅・高さと seed を受け取り、RGBA バッファのポインタを返す
#[no_mangle]
pub extern "C" fn render(width: u32, height: u32, _seed: u32) -> *mut u8 {
    let mut pix = Pixmap::new(width, height).expect("canvas");

    let mut pb = PathBuilder::new();
    let radius = (width.min(height) as f32) * 0.3;
    pb.push_circle((width as f32)/2.0, (height as f32)/2.0, radius);
    let path = pb.finish().unwrap();

    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(255, 0, 0, 255));

    pix.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        tiny_skia::Transform::identity(),
        None,
    );

    let data = pix.take();
    let len = data.len();
    let layout = Layout::array::<u8>(len).unwrap();
    let ptr = unsafe { alloc(layout) };
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, len);
    }
    ptr
}

/// メモリを解放する関数
#[no_mangle]
pub extern "C" fn free_buffer(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        let layout = Layout::array::<u8>(len).unwrap();
        unsafe {
            dealloc(ptr, layout);
        }
    }
}

