use std::net::TcpStream;
use std::io::{Read, Write};
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
    let mut stream = TcpStream::connect(worker_addr)
        .expect("No se pudo conectar al worker");

    let task = Task { iterations };

    let serialized = serde_json::to_vec(&task).unwrap();
    stream.write_all(&serialized).unwrap();

    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();

    serde_json::from_slice(&buffer).unwrap()
}

fn main() {

    // Simulación local: un worker en tu máquina
    let workers = vec![
        "127.0.0.1:9000",
    ];

    let total_iterations: u64 = 1_000_000;
    let iterations_per_worker = total_iterations / workers.len() as u64;

    let mut handles = Vec::new();

    for worker in workers {
        let worker_addr = worker.to_string();

        let handle = thread::spawn(move || {
            println!("📡 Enviando tarea a {}", worker_addr);
            send_task(&worker_addr, iterations_per_worker)
        });

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

    println!("==============================");
    println!("π aproximado = {}", pi);
    println!("==============================");
}
