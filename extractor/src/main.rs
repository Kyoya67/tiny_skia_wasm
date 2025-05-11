use wasmtime::{Engine, Store, Module, Instance};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    // 画像サイズ
    let width = 256u32;
    let height = 256u32;
    let size = (width * height * 4) as usize;

    // WASMモジュールをロード
    let engine = Engine::default();
    let module = Module::from_file(&engine, "../tiny_circle_wasm/target/wasm32-wasip1/release/tiny_circle_wasm.wasm")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // エクスポート取得
    let render = instance.get_typed_func::<(u32, u32, u32), u32>(&mut store, "render")?;
    let free_buffer = instance.get_typed_func::<(u32, u32), ()>(&mut store, "free_buffer")?;
    let memory = instance
        .get_export(&mut store, "memory")
        .and_then(|e| e.into_memory())
        .expect("memory export");

    // 画像データ生成
    let ptr = render.call(&mut store, (width, height, 0))?;

    // メモリから画像データ取得
    let data = memory
        .data(&store)
        .get(ptr as usize..(ptr as usize + size))
        .ok_or_else(|| anyhow::anyhow!("failed to read memory"))?;

    // ファイルに保存
    let mut file = File::create("output.raw")?;
    file.write_all(data)?;

    // メモリ解放
    free_buffer.call(&mut store, (ptr, size as u32))?;

    println!("output.raw を生成しました");
    Ok(())
}