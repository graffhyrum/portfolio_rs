use tracing::info;

fn get_port() -> u16 {
    let default_port = 8000;
    match dotenv::var("HTTPD_PORT") {
        Ok(port) => port.parse().unwrap_or(default_port),
        Err(_) => {
            info!("PORT not set, defaulting to {}", default_port);
            default_port
        }
    }
}

pub fn get_address() -> std::net::SocketAddr {
    let port = get_port();
    match dotenv::var("HTTPD_ADDRESS") {
        Ok(address) => {
            info!("ADDRESS set to {}", address);
            address.parse().unwrap_or(([0, 0, 0, 0], port).into())
        }
        Err(_) => {
            let default_address = [0, 0, 0, 0];
            info!("ADDRESS not set, defaulting to {:?}", default_address);
            ([0, 0, 0, 0], port).into()
        }
    }
}
