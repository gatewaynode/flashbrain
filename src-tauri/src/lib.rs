use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::env;
use rfd::FileDialog;

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub lesson_id: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub seconds_per_word: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingData {
    pub meta: Meta,
    pub items: Vec<TrainingItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingItem {
    pub title: String,
    pub acronym: String,
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
    pub speed: u32,
}

#[derive(Debug, Serialize)]
pub struct LearningPath {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct FileDialogResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
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
    let mut total_directories = 0;
    let mut directories_with_json = 0;
    let mut directories_with_training_json = 0;
    let mut directories_with_lesson_json = 0;
    let mut directories_without_json = 0;

    // Read all directories in static/classes
    for entry in fs::read_dir(classes_dir).map_err(|e| format!("Failed to read classes directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            total_directories += 1;
            let _dir_name = path.file_name()
                .and_then(|name| name.to_str())
                .ok_or("Invalid directory name")?;
            
            println!("Found directory: {}", _dir_name);
            
            // Look for lesson.json in this directory (skip training.json until Story 13)
            let lesson_json_path = path.join("lesson.json");
            
            let json_path = if lesson_json_path.exists() {
                directories_with_lesson_json += 1;
                directories_with_json += 1;
                println!("Found lesson.json in: {}", lesson_json_path.display());
                Some(lesson_json_path)
            } else {
                directories_without_json += 1;
                println!("No lesson.json found in: {}", path.display());
                None
            };
            
            if let Some(json_path) = json_path {
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
                    id: _dir_name.to_string(),
                    title: training_data.meta.title,
                    date: training_data.meta.date,
                    description: training_data.meta.description,
                });
            }
        }
    }

    // Comprehensive logging summary
    println!("=== LEARNING PATHS SUMMARY ===");
    println!("Total directories found: {}", total_directories);
    println!("Directories with JSON files: {}", directories_with_json);
    println!("  - Directories with lesson.json: {}", directories_with_lesson_json);
    println!("  - Directories with training.json: 0 (skipped until Story 13)");
    println!("Directories without JSON files: {}", directories_without_json);
    println!("Total learning paths successfully loaded: {}", learning_paths.len());
    println!("=================================");

    Ok(learning_paths)
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

    // Look for both training.json and lesson.json in this directory
    let training_json_path = class_path.join("training.json");
    let lesson_json_path = class_path.join("lesson.json");
    
    let json_path = if training_json_path.exists() {
        println!("Found training.json in class '{}'", class_id);
        training_json_path
    } else if lesson_json_path.exists() {
        println!("Found lesson.json in class '{}'", class_id);
        lesson_json_path
    } else {
        return Err(format!("Neither training.json nor lesson.json found in class '{}'", class_id));
    };

    println!("Loading training data from: {}", json_path.display());
    
    let json_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Failed to read {}: {}", json_path.display(), e))?;
    
    let training_data: TrainingData = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse JSON in {}: {}", json_path.display(), e))?;
    
    println!("Successfully loaded training data for: {}", training_data.meta.title);
    Ok(training_data)
}

#[tauri::command]
fn test_json_parsing() -> Result<String, String> {
    let json_content = r#"{
  "meta": {
    "class_id": "test-1",
    "title": "Marcus Aurelius Quotes",
    "date": "2025-06-29",
    "description": "This is a test of the flashbrain app. It is a simple app that allows you to create flashcards with images and text. The app will then flash the image and text for a given duration and speed.",
    "seconds_per_word": 0.5
  },
  "items": [
    {
      "item_id": "1",
      "text": "Men exist for the sake of one another. Teach them then or bear with them.",
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
fn test_lesson_discrepancy() -> Result<String, String> {
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
            break;
        }
    }

    let classes_dir = match classes_dir {
        Some(dir) => dir,
        None => {
            return Err("Could not find static/classes directory".to_string());
        }
    };

    let mut total_directories = 0;
    let mut directories_with_json = 0;
    let mut directories_with_training_json = 0;
    let mut directories_with_lesson_json = 0;
    let mut directories_without_json = 0;
    let mut successful_parses = 0;
    let mut failed_parses = 0;

    // Read all directories in static/classes
    for entry in fs::read_dir(classes_dir).map_err(|e| format!("Failed to read classes directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            total_directories += 1;
            let _dir_name = path.file_name()
                .and_then(|name| name.to_str())
                .ok_or("Invalid directory name")?;
            
            // Look for both training.json and lesson.json in this directory
            let training_json_path = path.join("training.json");
            let lesson_json_path = path.join("lesson.json");
            
            let json_path = if training_json_path.exists() {
                directories_with_training_json += 1;
                directories_with_json += 1;
                Some(training_json_path)
            } else if lesson_json_path.exists() {
                directories_with_lesson_json += 1;
                directories_with_json += 1;
                Some(lesson_json_path)
            } else {
                directories_without_json += 1;
                None
            };
            
            if let Some(json_path) = json_path {
                match fs::read_to_string(&json_path) {
                    Ok(json_content) => {
                        match serde_json::from_str::<TrainingData>(&json_content) {
                            Ok(_) => {
                                successful_parses += 1;
                            }
                            Err(e) => {
                                failed_parses += 1;
                                println!("Failed to parse JSON in {}: {}", json_path.display(), e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to read {}: {}", json_path.display(), e);
                    }
                }
            }
        }
    }

    // Create detailed report
    let report = format!(
        "=== LESSON DISCREPANCY TEST REPORT ===\n\
         Total directories found: {}\n\
         Directories with JSON files: {}\n\
           - Directories with training.json: {}\n\
           - Directories with lesson.json: {}\n\
         Directories without JSON files: {}\n\
         Successful JSON parses: {}\n\
         Failed JSON parses: {}\n\
         Expected lessons (directories with JSON): {}\n\
         Actual lessons (successful parses): {}\n\
         Discrepancy: {}\n\
         ======================================",
        total_directories,
        directories_with_json,
        directories_with_training_json,
        directories_with_lesson_json,
        directories_without_json,
        successful_parses,
        failed_parses,
        directories_with_json,
        successful_parses,
        if successful_parses == directories_with_json {
            "None - All JSON files parse successfully".to_string()
        } else {
            format!("{} lessons missing or have parsing errors", directories_with_json - successful_parses)
        }
    );

    println!("{}", report);
    Ok(report)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn open_file_dialog(title: String, filters: Vec<(String, Vec<String>)>) -> FileDialogResult {
    println!("Opening file dialog with title: {}", title);
    
    let mut dialog = FileDialog::new()
        .set_title(&title);
    
    // Add filters if provided
    for (name, extensions) in filters {
        dialog = dialog.add_filter(&name, &extensions);
    }
    
    match dialog.pick_file() {
        Some(path) => {
            let file_path = path.to_string_lossy().to_string();
            println!("File selected: {}", file_path);
            FileDialogResult {
                success: true,
                file_path: Some(file_path),
                error: None,
            }
        }
        None => {
            println!("File dialog was cancelled");
            FileDialogResult {
                success: false,
                file_path: None,
                error: Some("Dialog was cancelled".to_string()),
            }
        }
    }
}

#[tauri::command]
fn save_file_dialog(title: String, filters: Vec<(String, Vec<String>)>) -> FileDialogResult {
    println!("Opening save file dialog with title: {}", title);
    
    let mut dialog = FileDialog::new()
        .set_title(&title);
    
    // Add filters if provided
    for (name, extensions) in filters {
        dialog = dialog.add_filter(&name, &extensions);
    }
    
    match dialog.save_file() {
        Some(path) => {
            let file_path = path.to_string_lossy().to_string();
            println!("Save file selected: {}", file_path);
            FileDialogResult {
                success: true,
                file_path: Some(file_path),
                error: None,
            }
        }
        None => {
            println!("Save file dialog was cancelled");
            FileDialogResult {
                success: false,
                file_path: None,
                error: Some("Dialog was cancelled".to_string()),
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, get_learning_paths, test_json_parsing, load_training_data, open_file_dialog, save_file_dialog, test_lesson_discrepancy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_dialog_result_serialization() {
        let result = FileDialogResult {
            success: true,
            file_path: Some("/path/to/file.json".to_string()),
            error: None,
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("file_path"));
        assert!(json.contains("/path/to/file.json"));
    }

    #[test]
    fn test_file_dialog_result_cancelled() {
        let result = FileDialogResult {
            success: false,
            file_path: None,
            error: Some("Dialog was cancelled".to_string()),
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("Dialog was cancelled"));
    }
}
