use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub class_id: String,
    pub title: String,
    pub date: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingData {
    pub meta: Meta,
    pub items: Vec<TrainingItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingItem {
    pub item_id: String,
    pub text: String,
    pub image: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    pub payload: ActionPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionPayload {
    pub duration: u32,
    pub speed: u32,
}

#[derive(Debug, Serialize)]
pub struct LearningPath {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: String,
}

#[tauri::command]
fn get_learning_paths() -> Result<Vec<LearningPath>, String> {
    // Try multiple possible paths for the classes directory
    let possible_paths = vec![
        "static/classes",
        "../static/classes",
        "../../static/classes",
        "./static/classes",
    ];

    let paths_for_debug = possible_paths.clone();
    let mut classes_dir = None;
    
    for path_str in &possible_paths {
        let path = Path::new(path_str);
        if path.exists() {
            classes_dir = Some(path);
            println!("Found classes directory at: {}", path.display());
            break;
        }
    }

    let classes_dir = match classes_dir {
        Some(dir) => dir,
        None => {
            // Print current working directory for debugging
            let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("unknown").to_path_buf());
            println!("Current working directory: {}", current_dir.display());
            println!("Available paths tried: {:?}", paths_for_debug);
            return Err("Could not find static/classes directory".to_string());
        }
    };

    let mut learning_paths = Vec::new();

    // Read all directories in static/classes
    for entry in fs::read_dir(classes_dir).map_err(|e| format!("Failed to read classes directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name()
                .and_then(|name| name.to_str())
                .ok_or("Invalid directory name")?;
            
            println!("Found directory: {}", dir_name);
            
            // Look for training.json in this directory
            let json_path = path.join("training.json");
            
            if json_path.exists() {
                println!("Found training.json in: {}", json_path.display());
                
                let json_content = fs::read_to_string(&json_path)
                    .map_err(|e| format!("Failed to read {}: {}", json_path.display(), e))?;
                
                println!("JSON content length: {} characters", json_content.len());
                println!("JSON content preview: {}", &json_content[..json_content.len().min(200)]);
                
                let training_data: TrainingData = serde_json::from_str(&json_content)
                    .map_err(|e| {
                        println!("JSON parsing error: {}", e);
                        format!("Failed to parse JSON in {}: {}", json_path.display(), e)
                    })?;
                
                println!("Successfully parsed training data for: {}", training_data.meta.title);
                
                learning_paths.push(LearningPath {
                    id: dir_name.to_string(),
                    title: training_data.meta.title,
                    date: training_data.meta.date,
                    description: training_data.meta.description,
                });
            } else {
                println!("No training.json found in: {}", path.display());
            }
        }
    }

    println!("Total learning paths found: {}", learning_paths.len());
    Ok(learning_paths)
}

#[tauri::command]
fn test_json_parsing() -> Result<String, String> {
    let json_content = r#"{
  "meta": {
    "title": "Marcus Aurelius Quotes",
    "date": "2025-06-29",
    "description": "This is a test of the flashbrain app.  It is a simple app that allows you to create flashcards with images and text.  The app will then flash the image and text for a given duration and speed."
  },
  "items": [
    {
      "text": "Men exist for the sake of one another.  Teach them then or bear with them.",
      "image": "/static/classes/test-1/test_pattern.png",
      "actions": [
        {
          "type": "flash",
          "payload": {
            "duration": 85, "speed": 11
          }
        }
      ]
    }
  ]
}"#;

    match serde_json::from_str::<TrainingData>(json_content) {
        Ok(data) => Ok(format!("Successfully parsed: {}", data.meta.title)),
        Err(e) => Err(format!("JSON parsing failed: {}", e)),
    }
}

#[tauri::command]
fn load_training_data(class_id: String) -> Result<TrainingData, String> {
    // Try multiple possible paths for the classes directory
    let possible_paths = vec![
        "static/classes",
        "../static/classes",
        "../../static/classes",
        "./static/classes",
    ];

    let mut classes_dir = None;
    
    for path_str in &possible_paths {
        let path = Path::new(path_str);
        if path.exists() {
            classes_dir = Some(path);
            println!("Found classes directory at: {}", path.display());
            break;
        }
    }

    let classes_dir = match classes_dir {
        Some(dir) => dir,
        None => {
            return Err("Could not find static/classes directory".to_string());
        }
    };

    // Look for the specific class directory
    let class_path = classes_dir.join(&class_id);
    if !class_path.exists() {
        return Err(format!("Class directory '{}' not found", class_id));
    }

    // Look for training.json in this directory
    let json_path = class_path.join("training.json");
    
    if !json_path.exists() {
        return Err(format!("training.json not found in class '{}'", class_id));
    }

    println!("Loading training data from: {}", json_path.display());
    
    let json_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Failed to read {}: {}", json_path.display(), e))?;
    
    let training_data: TrainingData = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse JSON in {}: {}", json_path.display(), e))?;
    
    println!("Successfully loaded training data for: {}", training_data.meta.title);
    Ok(training_data)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_learning_paths, test_json_parsing, load_training_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
