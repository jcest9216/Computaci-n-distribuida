use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Task {
    iterations: u64,
}

#[derive(Deserialize)]
struct ResultMsg {
    inside: u64,
    total: u64,
}

fn send_task(worker_addr: &str, iterations: u64) -> ResultMsg {
    println!("Conectando a worker en {}", worker_addr);
    let mut stream = TcpStream::connect(worker_addr)
        .expect(&format!("No se pudo conectar al worker en {}", worker_addr));

    let task = Task { iterations };

    let serialized = serde_json::to_vec(&task).unwrap();
    stream.write_all(&serialized).unwrap();
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();

    serde_json::from_slice(&buffer).unwrap()
}

fn main() {
    let workers = vec![
        // julio
        "10.0.0.2:9000",
        "10.0.0.3:9000",
        "10.0.0.4:9000",
        "10.0.0.5:9000",
        // // yami
        // "10.0.0.6:9000",
        // "10.0.0.7:9000",
        // "10.0.0.8:9000",
        // "10.0.0.9:9000",
        // "10.0.0.10:9000",
        // // esther
        // "10.0.0.11:9000",
        // "10.0.0.12:9000",
        // "10.0.0.13:9000",
        // "10.0.0.14:9000",
        // "10.0.0.15:9000",
        // // levy
        // "10.0.0.16:9000",
        // "10.0.0.17:9000",
        // "10.0.0.18:9000",
        // "10.0.0.19:9000",
        // "10.0.0.20:9000",
    ];

    let total_iterations: u64 = 1_000_000;
    let iterations_per_worker = total_iterations / workers.len() as u64;

    let mut handles = vec![];

    for worker in workers {
        let worker_addr = worker.to_string();

        let handle = thread::spawn(move || send_task(&worker_addr, iterations_per_worker));

        handles.push(handle);
    }

    let mut total_inside = 0;
    let mut total_points = 0;

    for handle in handles {
        let result = handle.join().unwrap();
        total_inside += result.inside;
        total_points += result.total;
    }

    let pi = 4.0 * (total_inside as f64) / (total_points as f64);

    println!("π aproximado = {}", pi);
}

