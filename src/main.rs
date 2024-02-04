use std::collections::HashMap;

use base64::Engine;
use clap::Parser;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DockerConfig {
    auths: HashMap<String, DockerAuth>,
}

#[derive(Debug, Serialize)]
struct DockerAuth {
    username: String,
    password: String,
    auth: String,
}

impl DockerAuth {
    pub fn new(username: &str, password: &str) -> Self {
        let auth =
            base64::engine::general_purpose::STANDARD.encode(format!("{username}:{password}"));

        Self {
            username: username.to_string(),
            password: password.to_string(),
            auth,
        }
    }
}

#[derive(Parser)]
struct Args {
    host: String,

    #[arg(short, long)]
    username: String,

    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();

    let docker_auth = DockerAuth::new(&args.username, &args.password);

    let docker_config = DockerConfig {
        auths: HashMap::from_iter([(args.host, docker_auth)]),
    };

    let docker_config_json = serde_json::to_string(&docker_config).unwrap();

    // println!("{docker_config_json}");

    let encoded = base64::engine::general_purpose::STANDARD.encode(docker_config_json);

    println!("{encoded}");
}

#[test]
fn test_main() {
    let a = base64::engine::general_purpose::STANDARD
        .encode("robot$adge+hrghe:yLfLfihLZdMwKIfnFO3fdYm3BE3j5rqhB");

    let b = base64::engine::general_purpose::STANDARD
        .decode("cm9ib3QkYWRnZStocmdoZTp5TGZMZmloTFpkTXdLSWZuRk8zZmRZbTNCRTNqNXJxaEI=")
        .ok()
        .and_then(|x| String::from_utf8(x).ok())
        .unwrap();

    let c = base64::engine::general_purpose::STANDARD
        .decode(&a)
        .ok()
        .and_then(|x| String::from_utf8(x).ok())
        .unwrap();

    assert_eq!(b, c);
}
