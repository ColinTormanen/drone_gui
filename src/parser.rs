use chrono::Local;

use crate::protocol::TelemetryPacket;
use crate::telemetry::{ReceivedMessage, TelemetryData};

pub fn parse_rcv(line: &str) -> Option<ReceivedMessage> {
    let parts: Vec<&str> = line.strip_prefix("+RCV=")?.split(",").collect();

    let address: u32 = parts[0].parse().ok()?;
    let length: u32 = parts[1].parse().ok()?;
    let message = parts[2].to_string();
    let rssi: i32 = parts[3].parse().ok()?;
    let snr: i32 = parts[4].parse().ok()?;

    Some(ReceivedMessage {
        from: address,
        length,
        message,
        rssi,
        snr,
        time: Local::now(),
    })
}

/// Parse telemetry from serial data
/// Format: "TELEM:roll:pitch:yaw:roll_p:roll_i:roll_d:pitch_p:pitch_i:pitch_d:yaw_p:yaw_i:yaw_d:alt:voltage"
/// Each field is a float formatted as [sign]whole.decimal (e.g., "0.123", "-1.456")
pub fn parse_telemetry(line: &str) -> Option<TelemetryData> {
    let mut parts = line.splitn(2, ':');
    let header = parts.next()?;
    let hex = parts.next()?;

    if header != "T" {
        return None;
    }

    todo!()
}

/// Parse log message from serial data
/// Format: "LOG:message text here"
pub fn parse_log(line: &str) -> Option<String> {
    line.strip_prefix("LOG:").map(str::to_string)
}
