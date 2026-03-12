use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::thread;
use std::time::Duration;

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

fn send_task(worker_addr: &str, iterations: u64) -> Option<ResultMsg> {
    println!("Conectando a worker en {}", worker_addr);

    let addr: SocketAddr = worker_addr.parse().unwrap();
    let timeout = Duration::from_secs(2);

    let mut stream = match TcpStream::connect_timeout(&addr, timeout) {
        Ok(stream) => stream,
        Err(e) => {
            println!("⚠️ No se pudo conectar a {}: {}", worker_addr, e);
            return None;
        }
    };

    let task = Task { iterations };

    let serialized = match serde_json::to_vec(&task) {
        Ok(data) => data,
        Err(e) => {
            println!("⚠️ Error serializando tarea: {}", e);
            return None;
        }
    };

    if let Err(e) = stream.write_all(&serialized) {
        println!("⚠️ Error enviando tarea a {}: {}", worker_addr, e);
        return None;
    }

    let _ = stream.shutdown(std::net::Shutdown::Write);

    let mut buffer = Vec::new();

    if let Err(e) = stream.read_to_end(&mut buffer) {
        println!("⚠️ Error leyendo respuesta de {}: {}", worker_addr, e);
        return None;
    }

    match serde_json::from_slice(&buffer) {
        Ok(result) => Some(result),
        Err(e) => {
            println!("⚠️ Error deserializando respuesta de {}: {}", worker_addr, e);
            None
        }
    }
}

fn main() {

    let workers = vec![
        // ===== WORKERS REALES =====
        // julio
        // "10.0.0.2:9000",
        // "10.0.0.3:9000",
        // "10.0.0.4:9000",
        // "10.0.0.5:9000",

        // yami
        // "10.0.0.6:9000",
        // "10.0.0.7:9000",
        // "10.0.0.8:9000",
        // "10.0.0.9:9000",
        // "10.0.0.10:9000",

        // esther
        // "10.0.0.11:9000",
        // "10.0.0.12:9000",
        // "10.0.0.13:9000",
        // "10.0.0.14:9000",
        // "10.0.0.15:9000",

        // levy
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

        let handle = thread::spawn(move || {
            let result = send_task(&worker_addr, iterations_per_worker);
            (worker_addr, result)
        });

        handles.push(handle);
    }

    let mut total_inside = 0;
    let mut total_points = 0;
    let mut active_workers = 0;

    for handle in handles {
        match handle.join() {
            Ok((worker_addr, Some(result))) => {
                println!("✅ Worker {} respondió correctamente", worker_addr);

                total_inside += result.inside;
                total_points += result.total;
                active_workers += 1;
            }

            Ok((worker_addr, None)) => {
                println!("❌ Worker {} falló o no respondió", worker_addr);
            }

            Err(_) => {
                println!("❌Error en thread");
            }
        }
    }

    if total_points == 0 {
        println!("❌ Ningún worker respondió.");
        return;
    }

    let pi = 4.0 * (total_inside as f64) / (total_points as f64);

    println!("--------------------------------");
    println!("Workers activos: {}", active_workers);
    println!("Puntos totales: {}", total_points);
    println!("π aproximado = {}", pi);
}

