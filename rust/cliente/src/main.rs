use reqwest::Client;
use shared::{Task, TaskResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let coordinator_url = "http://10.0.0.1:3000";
    
    
    let task: Task = client
        .get(format!("{}/task", coordinator_url))
        .send()
        .await?
        .json()
        .await?;

    println!("Tarea recibida: id={}, tarea={}", task.id, task.payload);

    let result = TaskResult {
        task_id: task.id,
        result: format!("procesado: {}", task.payload),
    };

    let response = client
        .post(format!("{}/result", coordinator_url))
        .json(&result)
        .send()
        .await?
        .text()
        .await?;

    println!("Respuesta: {}", response);

    Ok(())
}
