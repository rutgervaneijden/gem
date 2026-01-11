use std::io::Result;
use std::io::Write;
use std::net::TcpStream;
use std::net::UdpSocket;

fn main() {
    let nmea_bridge_result = nmea_bridge("0.0.0.0:55551", "127.0.0.1:1234");
    println!("{:?}", nmea_bridge_result)
}

fn keyboard_bridge() {

}

fn nmea_bridge(nmea_in_address: &str, nmea_out_address: &str) -> Result<()> {
    let nmea_in_socket = UdpSocket::bind(nmea_in_address)?;
    let mut buf = [0; 1024];
    let mut nmea_out_socket = TcpStream::connect(nmea_out_address)?;
    loop {
        let (_len, _src) = nmea_in_socket.recv_from(&mut buf)?;
        nmea_out_socket.write_all(&buf)?;
    }
}

fn tune_grabber() {

}

fn gain_grabber() {

}

fn sea_grabber() {

}

fn rain_grabber() {

}