use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Task {
    iterations: u64,
}

#[derive(Serialize)]
struct ResultMsg {
    inside: u64,
    total: u64,
}

fn montecarlo(iterations: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut inside = 0;

    for _ in 0..iterations {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }

    inside
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();

    // Leer datos del coordinador
    match stream.read_to_end(&mut buffer) {
        Ok(_) => {
            let task: Result<Task, _> = serde_json::from_slice(&buffer);

            match task {
                Ok(task) => {
                    println!("Tarea recibida: {} iteraciones", task.iterations);

                    let inside = montecarlo(task.iterations);

                    let result = ResultMsg {
                        inside,
                        total: task.iterations,
                    };

                    let response = serde_json::to_vec(&result).unwrap();

                    if let Err(e) = stream.write_all(&response) {
                        eprintln!("Error enviando respuesta: {}", e);
                    } else {
                        println!("La tarea ha sido completada");
                        println!("Resultado: {}", result.inside);
                        println!("Enviando resultado...");
                    }
                }
                Err(e) => {
                    eprintln!("Error parseando tarea: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error leyendo stream: {}", e);
        }
    }
}

fn main() {
    let listener =
        TcpListener::bind("0.0.0.0:9000").expect("No se pudo iniciar el worker en el puerto 9000");

    println!("Worker escuchando en puerto 9000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Cada conexión se maneja en un hilo
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error aceptando conexión: {}", e);
            }
        }
    }
}

