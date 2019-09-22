use std::io::*;
use std::time::Duration;

extern crate clap;
extern crate serialport;

use std::io::Write;

use clap::{App, AppSettings, Arg};
use colored::*;
use serialport::prelude::*;

fn main() -> Result<()> {
    let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at")
                .use_delimiter(false)
                .required(true),
        )
        .get_matches();
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap();

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    let mut port = serialport::open_with_settings(&port_name, &settings)?;
    // let mut serial_buf: Vec<u8> = vec![0; 1000];
    println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
    let mut reader = BufReader::new(&mut port);
    loop {
        // let mut slice = serial_buf.as_mut_slice();
        // let mut start = 0;
        let mut serial_str = String::new();
        // match port.read(&mut slice[start..]) {
        match reader.read_line(&mut serial_str) {
            Ok(t) => {
                if t == 0 {
                    continue;
                }

                if serial_str.find("ok") == None && serial_str.find("Grbl") != None {
                    continue;
                }

                println!("{}", serial_str.trim());

                loop {
                    let mut input = String::new();
                    match stdin().read_line(&mut input) {
                        Ok(_) => {
                            let trimmed = input.as_str().trim();

                            // skip this
                            if trimmed.len() == 0 {
                                continue;
                            }
                            let line = format!("{}\n", trimmed);
                            print!("{} :: ", trimmed.white());

                            let bytes = line.as_bytes();
                            match reader.get_mut().write_all(&bytes[..]) {
                                Ok(_) => {
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("could not write to sp. Error: {}", e);
                                    break;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("Failed to read from stdin, Error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
