#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::io::{Write, Read};
use std::net::{TcpStream, SocketAddr};
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct ScanResult {
    ip: String,
    ping: u128,
    is_official: bool,
}

fn is_official_cf(ip: &str) -> bool {
    let prefixes = [
        "173.245.", "103.21.", "103.22.", "103.31.", "141.101.", "108.162.", 
        "190.93.", "188.114.", "197.234.", "198.41.", "162.158.", "104.16.", 
        "104.17.", "104.18.", "104.19.", "104.20.", "104.21.", "104.22.", 
        "104.23.", "104.24.", "172.64.", "131.0."
    ];
    prefixes.iter().any(|&p| ip.starts_with(p))
}

#[tauri::command]
fn scan_ips() -> Vec<ScanResult> {
    let ips = vec![
        "104.17.2.3", "104.18.5.6", "162.159.192.1", "188.114.97.3",
        "104.21.3.4", "172.64.1.2", "1.1.1.1", "8.8.8.8"
    ];
    
    let mut results = Vec::new();
    
    for ip in ips {
        if let Ok(addr) = format!("{}:443", ip).parse::<SocketAddr>() {
            let start = Instant::now();
            if TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok() {
                results.push(ScanResult {
                    ip: ip.to_string(),
                    ping: start.elapsed().as_millis(),
                    is_official: is_official_cf(ip),
                });
            }
        }
    }
    
    results.sort_by_key(|r| r.ping);
    results
}

#[tauri::command]
fn test_target(ip: String, target: String) -> Result<u128, String> {
    let addr: SocketAddr = format!("{}:80", ip).parse().map_err(|e| e.to_string())?;
    let start = Instant::now();
    
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(3)).map_err(|e| e.to_string())?;
    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", target);
    
    stream.write_all(request.as_bytes()).map_err(|e| e.to_string())?;
    
    let mut buffer = [0; 128];
    let _ = stream.read(&mut buffer);
    
    Ok(start.elapsed().as_millis())
}

// این بخش برای اجرای بدون مشکل روی گوشی اندروید اضافه شد
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_ips, test_target])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}