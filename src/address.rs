use tracing::info;

fn get_port() -> u16 {
    match dotenv::var("HTTPD_PORT") {
        Ok(port) => port.parse().unwrap_or(8080),
        Err(_) => {
            info!("PORT not set, defaulting to 8080");
            8080
        }
    }
}

pub fn get_address() -> std::net::SocketAddr {
    let port = get_port();
    match dotenv::var("HTTPD_ADDRESS") {
        Ok(address) => {
            info!("ADDRESS set to {}", address);
            address.parse().unwrap_or(([127, 0, 0, 1], port).into())
        }
        Err(_) => {
            let default_address = [127, 0, 0, 1];
            info!("ADDRESS not set, defaulting to {:?}", default_address);
            ([127, 0, 0, 1], port).into()
        }
    }
}
