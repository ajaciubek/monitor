use clap::Parser;
use core::time;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io::Read};
use sysconf::raw::sysconf;
// use syscon
#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pid: u32,
    #[arg(short, long)]
    time: u64,
}

fn get_times(pid: u32) -> (f32, u128) {
    let mut stat = fs::File::open(format!("/proc/{}/stat", pid)).unwrap();
    let mut data = String::new();
    let _ = stat.read_to_string(&mut data);
    let tokens: Vec<&str> = data.split(" ").collect();
    let utime = tokens[tokens.len() - 39].parse::<u32>().unwrap();
    let stime = tokens[tokens.len() - 39].parse::<u32>().unwrap();
    let current_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    ((utime + stime) as f32, current_ms)
}

fn main() {
    let args = Args::parse();
    let (prev_cpu_time, prev_ms) = get_times(args.pid);

    for i in 0..args.time {
        println!("{:.2}%", (i + 1) as f32 / args.time as f32 * 100.0);
        thread::sleep(time::Duration::from_secs(1));
    }

    let clock_ticks = sysconf(sysconf::SysconfVariable::ScClkTck).unwrap() as f32;
    let (crr_cpu_time, crr_ms) = get_times(args.pid);

    let elapsed_ms = (crr_ms - prev_ms) as f32;
    let cpu_time_ms = ((crr_cpu_time - prev_cpu_time) * 1000.0) / clock_ticks;
    let usage = 100.0 * cpu_time_ms / elapsed_ms;
    println!("usage: {:.2}", usage);
}
