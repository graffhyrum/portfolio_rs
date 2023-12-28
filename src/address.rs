use tracing::info;

fn get_port() -> u16 {
    let port = 8000;
    match dotenv::var("HTTPD_PORT") {
        Ok(dotenv_port) => dotenv_port
            .parse()
            .unwrap_or(port),
        Err(_) => {
            // if not set in dotenv, check the environment
            match std::env::var("PORT") {
                Ok(env_port) => {
                    info!("PORT set to {}", env_port);
                    env_port.parse()
                        .unwrap_or(port)
                }
                Err(_) => {
                    info!("PORT not set, defaulting to {}", port);
                    port
                }
            }
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
