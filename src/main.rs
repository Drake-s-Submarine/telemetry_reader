use std::io::Read;
use std::fs::File;

const TELEMETRY_PACKET_SIZE: usize = 32;

fn main() {
    let pipe_path = "/tmp/sub_tlm_socket";
    let mut pipe = File::open(pipe_path).unwrap();

    loop {
        read_pipe(&mut pipe);
    }
}

fn read_pipe(pipe: &mut File) {
    let mut buf = [0; TELEMETRY_PACKET_SIZE];

    let len = pipe.read(&mut buf).unwrap();

    if len < 1 { return; }

    match buf[len - 1] {
        0x0 => { 
            let packet = EnvironmentTelemetry::deserialize(&buf);
            println!("{:?}{}", packet, len);
        },

        0xF => {
            let packet = SystemTelemetry::deserialize(&buf);
            println!("{:?}", packet);
        },
        _ => eprintln!("Back packet ID: {}", buf[len-1]),
    };
}

#[derive(Debug)]
pub struct EnvironmentTelemetry {
    pub internal_temperature_c: u8,
    pub internal_humidity_percent: u8,
    pub is_stale: bool,
}

impl EnvironmentTelemetry {
    fn deserialize(buffer: &[u8; TELEMETRY_PACKET_SIZE]) -> Self {
       Self {
           internal_temperature_c: buffer[0],
           internal_humidity_percent: buffer[1],
           is_stale: buffer[2] < 1,
       } 
    }
}

#[derive(Debug)]
pub struct SystemTelemetry {
    tick_delta: u32,
    tick_idle_time: u32,
    total_tick_time: u32,
}

impl SystemTelemetry {
    fn deserialize(buffer: &[u8; TELEMETRY_PACKET_SIZE]) -> Self {
        let mut delta: [u8; 4] = [0; 4];
        let mut idle: [u8; 4] = [0; 4];
        let mut total: [u8; 4] = [0; 4];

        delta[0] = buffer[0];
        delta[1] = buffer[1];
        delta[2] = buffer[2];
        delta[3] = buffer[3];

        idle[0] = buffer[4];
        idle[1] = buffer[5];
        idle[2] = buffer[6];
        idle[3] = buffer[7];

        total[0] = buffer[8];
        total[1] = buffer[9];
        total[2] = buffer[10];
        total[3] = buffer[11];

        Self {
            tick_delta: u32::from_le_bytes(delta),
            tick_idle_time: u32::from_le_bytes(idle),
            total_tick_time: u32::from_le_bytes(total),
        }
    }
}
