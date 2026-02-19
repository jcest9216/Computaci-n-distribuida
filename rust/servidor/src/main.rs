use axum::{
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use shared::{Task, TaskResult};

#[derive(Clone)]
struct AppState {
    task_counter: Arc<Mutex<u64>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        task_counter: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/task", get(get_task))
        .route("/result", post(receive_result))
        .with_state(state);

    let addr = SocketAddr::from(([10, 0, 0, 1], 3000));
    println!("Escuchando en: http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}

// get
async fn get_task(state: axum::extract::State<AppState>) -> Json<Task> {
    let mut counter = state.task_counter.lock().unwrap();
    *counter += 1;
    let task_id = *counter;

    let dummy_task = Task {
        id: task_id,
        payload: format!("Tarea número {}: procesa este dato", task_id),
    };

    println!("Tarea asignada: id={}", task_id);
    println!("Payload: {}", dummy_task.payload);

    Json(dummy_task)
}

// post
async fn receive_result(
    Json(result): Json<TaskResult>,
) -> String {
    println!("Resultado recibido de la tarea con id {}: {}", result.task_id, result.result);
    println!("\n");

    "Resultado recibido correctamente".to_string()
}
