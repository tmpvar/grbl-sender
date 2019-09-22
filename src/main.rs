use std::io;
use std::time::Duration;
// use std::io::prelude::*;
use std::thread;

extern crate clap;
extern crate serialport;

use std::io::Write;

use clap::{App, AppSettings, Arg};
use colored::*;
use serialport::prelude::*;

fn main() {
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

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            // Clone the port
            let mut clone = port.try_clone().expect("Failed to clone");

            // Send out 4 bytes every second
            thread::spawn(move || loop {
                clone
                    .write(&['?' as u8])
                    .expect("Failed to write to serial port");
                thread::sleep(Duration::from_millis(1000));
            });

            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        io::stdout().write_all(&serial_buf[..t]).unwrap();

                        // skip status lines
                        if serial_buf[0] == '<' as u8 {
                            continue;
                        }

                        loop {
                            let mut input = String::new();
                            match io::stdin().read_line(&mut input) {
                                // TODO: if we read an empty line from stdin, discard this one and read
                                //       another.
                                Ok(_) => {
                                    let trimmed = input.as_str().trim();

                                    // skip this
                                    if trimmed.len() == 0 {
                                        continue;
                                    }
                                    let line = format!("{}\n", trimmed);
                                    println!("{}", trimmed.white());
                                    //input.push_str("\n");
                                    let bytes = line.as_bytes();
                                    match port.write_all(&bytes[..]) {
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
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

// fn main() {
//     for arg in env::args_os().skip(1) {
//         println!("open port {:?}", arg);
//         let mut port = serial::open(&arg).unwrap();
//         interact(&mut port).unwrap();
//     }
// }
//
// fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
//     port.reconfigure(&|settings| {
//
//         settings.set_baud_rate(serial::Baud115200)?;
//         settings.set_char_size(serial::Bits8);
//         settings.set_parity(serial::ParityNone);
//         settings.set_stop_bits(serial::Stop1);
//         settings.set_flow_control(serial::FlowNone);
//         Ok(())
//     })?;
//
//     port.set_timeout(Duration::from_millis(1000))?;
//     let mut header = String::new();
//
//     port.read_to_string(&mut header)?;
//
//     println!("read header {}", header);
//
//     loop {
//         let mut input = String::new();
//         let len = io::stdin().read_line(&mut input)?;
//         if len == 0 {
//             break;
//         }
//         println!("stdin: {}", input);
//
//         //let bytes = input.clone().into_bytes();
//
//         //port.write(&bytes)?;
//         port.write(b"GX100F100\n")?;
//
//         println!("send: {}", input);
//         // let mut buf: Vec<u8> = (0..255).collect();
//         let mut recv_line = String::new();
//         port.read_to_string(&mut recv_line)?;
//         // let recv_bytes = port.read(&mut buf[..])?;
//         // println!("recv: {:?}", String::from_utf8(buf));
//         println!("recv: {}", recv_line);
//     }
//
//     Ok(())
// }
