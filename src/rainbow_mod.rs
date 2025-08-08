use colored::Colorize;
use std::io::{self, Write};
use std::time::{Duration, Instant};

const RAINBOW_STOPS: [(u8,u8,u8); 7] = [
    (255,0,0),(255,128,0),(255,255,0),(0,255,0),(0,255,255),(0,0,255),(153,0,255)
];

pub fn rainbow(text: &str, base_offset: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() { return String::new(); }
    let len = chars.len();
    let denom = (len.max(2) - 1) as f32;
    let mut out = String::with_capacity(text.len() * 10);
    let last = RAINBOW_STOPS.len() - 1;

    for (i, ch) in chars.into_iter().enumerate() {
        let rolled = (i + base_offset) % len;
        let mut pos = rolled as f32 / denom;
        if pos >= 1.0 { pos = f32::from_bits(0x3F7FFFFF); } // 0.99999994
        let segf = pos * last as f32;
        let seg = segf.floor() as usize;
        let t = segf - seg as f32;

        let (sr, sg, sb) = RAINBOW_STOPS[seg];
        let (er, eg, eb) = RAINBOW_STOPS[(seg + 1).min(last)];
        let r = (sr as f32 + (er as f32 - sr as f32) * t).round() as u8;
        let g = (sg as f32 + (eg as f32 - sg as f32) * t).round() as u8;
        let b = (sb as f32 + (eb as f32 - sb as f32) * t).round() as u8;

        out.push_str(&ch.to_string().truecolor(r, g, b).to_string());
    }
    out
}

// 用于在动画结束时恢复光标
struct CursorGuard;
impl Drop for CursorGuard {
    fn drop(&mut self) {
        eprint!("\x1b[?25h\n");
        let _ = io::stderr().flush();
    }
}

/// 异步动画版：不阻塞运行时线程（用 tokio::time::sleep）
pub async fn animate_async(
    text: &str,
    fps: u32,
    step: usize,
    seconds: Option<u64>,
) -> io::Result<()> {
    let _guard = CursorGuard; // 作用域结束自动恢复光标

    // 隐藏光标
    eprint!("\x1b[?25l");
    io::stderr().flush()?;

    let frame = Duration::from_secs_f32(1.0 / fps.max(1) as f32);
    let start = Instant::now();
    let mut offset = 0usize;

    loop {
        print!("\r\x1b[2K{}", rainbow(text, offset));
        io::stdout().flush()?; // 这行是阻塞的，但很快；不影响其它 async 任务调度
        offset = offset.wrapping_add(step);

        if let Some(s) = seconds {
            if start.elapsed() >= Duration::from_secs(s) {
                break;
            }
        }
        tokio::time::sleep(frame).await; // 关键：异步睡眠
    }
    Ok(())
}
