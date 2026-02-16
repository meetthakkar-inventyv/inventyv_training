use crate::model::Project;
use std::fs;

pub async fn load_projects() -> Vec<Project> {
    tokio::task::spawn_blocking(|| {
        fs::read_to_string("projects.json")
            .map(|data| serde_json::from_str(&data).unwrap_or_default())
            .unwrap_or_default()
    })
    .await
    .unwrap_or_default()
}

pub async fn save_projects(projects: Vec<Project>) {
    let _ = tokio::task::spawn_blocking(move || {
        if let Ok(json) = serde_json::to_string_pretty(&projects) {
            let _ = fs::write("projects.json", json);
        }
    })
    .await;
}