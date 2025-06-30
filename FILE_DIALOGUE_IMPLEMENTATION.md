# File Dialogue Implementation

This document explains how native file dialogues are implemented in FlashBrain using the RFD (Rust File Dialog) crate.

## Overview

The application now uses native OS file dialogues on macOS (and other platforms) instead of the Tauri dialog plugin. This provides a better user experience with platform-native file picker interfaces.

## Implementation Details

### Backend (Rust)

#### Dependencies
- Added `rfd = "0.13"` to `src-tauri/Cargo.toml`

#### New Commands
Two new Tauri commands were added to `src-tauri/src/lib.rs`:

1. **`open_file_dialog`** - Opens a file picker dialog
   - Parameters: `title` (String), `filters` (Vec<(String, Vec<String>)>)
   - Returns: `FileDialogResult` with success status, file path, and error info

2. **`save_file_dialog`** - Opens a file save dialog
   - Parameters: `title` (String), `filters` (Vec<(String, Vec<String>)>)
   - Returns: `FileDialogResult` with success status, file path, and error info

#### Data Structures
```rust
#[derive(Debug, Serialize)]
pub struct FileDialogResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
}
```

### Frontend (Svelte)

#### Updated Functions
The `src/routes/create/+page.svelte` file was updated to use the new Rust commands:

1. **`handleFileSelect()`** - For editing existing lessons
   - Calls `invoke('open_file_dialog', {...})`
   - Handles JSON file selection with `.json` filter

2. **`handleImportFile()`** - For importing lessons
   - Calls `invoke('open_file_dialog', {...})`
   - Handles JSON file selection with `.json` filter

#### Error Handling
- Comprehensive error handling for both success and failure cases
- Fallback to manual path input if dialog fails
- User-friendly error messages
- Console logging for debugging

## Usage Examples

### Opening a File Dialog
```javascript
const result = await invoke('open_file_dialog', {
  title: 'Select Training JSON File',
  filters: [['JSON Files', ['json']]]
});

if (result.success && result.file_path) {
  // File was selected
  console.log('Selected file:', result.file_path);
} else {
  // Dialog was cancelled or failed
  console.log('Error:', result.error);
}
```

### Opening a Save Dialog
```javascript
const result = await invoke('save_file_dialog', {
  title: 'Save Training JSON File',
  filters: [['JSON Files', ['json']]]
});

if (result.success && result.file_path) {
  // Save location was selected
  console.log('Save location:', result.file_path);
} else {
  // Dialog was cancelled or failed
  console.log('Error:', result.error);
}
```

## Benefits

1. **Native Experience** - Uses platform-native file dialogues
2. **Better UX** - Familiar interface for users
3. **Reliability** - More stable than web-based alternatives
4. **Performance** - Direct OS integration
5. **Consistency** - Matches user's OS expectations

## Testing

Unit tests are included to verify:
- JSON serialization of dialog results
- Proper handling of cancelled dialogs
- Error message formatting

Run tests with:
```bash
cd src-tauri && cargo test
```

## Troubleshooting

### Common Issues

1. **Dialog doesn't appear**
   - Check console for error messages
   - Verify RFD crate is properly imported
   - Ensure Tauri command is registered

2. **File filters not working**
   - Verify filter format: `[['Name', ['ext1', 'ext2']]]`
   - Check that extensions are lowercase

3. **Permission errors**
   - Ensure app has proper permissions
   - Check macOS security settings

### Debug Logging

The implementation includes extensive console logging:
- Dialog opening attempts
- User selections
- Error conditions
- Fallback behavior

Check browser console and Rust logs for debugging information. 