use clap::Args;
use colored::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::net::{IpAddr, ToSocketAddrs};
use std::thread::sleep;
use std::time::Duration;

use ping_rs::{send_ping, PingOptions};

use crate::utils::{cat_face, get_time_period, load_waifu_lines, rainbow, speak, AppResult, VoiceLine, RAINBOW_STOPS};

/// 🏓 `waifu ping` 子命令参数
#[derive(Args)]
pub struct PingArgs {
    /// 要 ping 的目标地址（域名或 IP）
    pub target: String,

    /// ping 次数（默认 4 次）
    #[arg(short, long, default_value_t = 4)]
    pub count: u32,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

/// 🏁 子命令入口
pub fn run_ping(args: PingArgs) -> AppResult {
    println!("{}", "🌸 Waifu Ping 正在启动~".magenta().bold());

    let Some(ip) = resolve_host_to_ip(&args.target) else {
        eprintln!("{}", "呜呜……主人，找不到这个地址~ >_<".red());
        return Err(format!("无法解析目标地址：{}", args.target));
    };

    println!(
        "{} {} → {}\n",
        "🎯 目标地址:".bold(),
        args.target.cyan(),
        ip.to_string().yellow()
    );

    for i in 1..=args.count {
        print!("{} ", format!("[{}/{}]", i, args.count).dimmed());

        match ping_once(ip) {
            Ok(ms) => {
                get_success_line(ms, ip, args.miao)?;
            }
            Err(_) => {
                get_fail_line(args.miao)?;
            }
        }

        sleep(Duration::from_secs(1));
    }

    println!("\n{}", "🌟 Waifu 任务结束~ 记得夸夸我嘛~".magenta());
    Ok(())
}

/// 🌐 域名解析为 IP
fn resolve_host_to_ip(host: &str) -> Option<IpAddr> {
    let addr = (host, 0).to_socket_addrs().ok()?.next()?;
    Some(addr.ip())
}

/// 🧪 单次 ping
fn ping_once(addr: IpAddr) -> Result<u32, String> {
    let data = [1, 2, 3, 4];
    let timeout = Duration::from_secs(1);
    let options = PingOptions {
        ttl: 128,
        dont_fragment: true,
    };

    match send_ping(&addr, timeout, &data, Some(&options)) {
        Ok(reply) => Ok(reply.rtt),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// ✅ ping 成功回应
fn get_success_line(ms: u32, ip: IpAddr, with_voice: bool) -> AppResult {
    let ip_str = ip.to_string();
    let ms_str = ms.to_string();
    let time = get_time_period();

    let lines = load_waifu_lines()?;
    let pool = lines
        .ping
        .success
        .get(time)
        .or_else(|| lines.ping.success.get("default"))
        .ok_or_else(|| "没有找到成功语音".to_string())?;

    let VoiceLine { cn, jp } = pool
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| "成功语音列表为空".to_string())?;
    let cn_line = cn.replace("{ip}", &ip_str).replace("{ms}", &ms_str);
    let jp_line = jp.replace("{ip}", &ip_str).replace("{ms}", &ms_str);

    let offset = rand::thread_rng().gen_range(0..RAINBOW_STOPS.len() as u8);
    println!(
        "{} {}",
        cat_face(&lines).bright_yellow(),
        rainbow(&cn_line, offset)
    );

    if with_voice {
        speak(&jp_line);
    }

    Ok(())
}

/// ❌ ping 失败回应
fn get_fail_line(with_voice: bool) -> AppResult {
    let time = get_time_period();

    let lines = load_waifu_lines()?;
    let pool = lines
        .ping
        .fail
        .get(time)
        .or_else(|| lines.ping.fail.get("default"))
        .ok_or_else(|| "没有找到失败语音".to_string())?;

    let VoiceLine { cn, jp } = pool
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| "失败语音列表为空".to_string())?;
    let offset = rand::thread_rng().gen_range(0..6);
    println!("{} {}", cat_face(&lines).bright_red(), rainbow(cn, offset));

    if with_voice {
        speak(jp);
    }

    Ok(())
}
