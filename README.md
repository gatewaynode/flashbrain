# FlashBrain

A Tauri desktop application for creating and using flashcards with images and text.

## Features

- Create and edit flashcard lessons
- Import existing lessons
- Training mode with image and text display
- Crossfade animations between image and text
- Word-by-word highlighting during training
- Native file dialogues for lesson management

## JSON Schema

The application uses JSON files to store lesson data. Here's the schema:

```json
{
  "meta": {
    "class_id": "string",
    "title": "string",
    "date": "string (YYYY-MM-DD)",
    "description": "string",
    "seconds_per_word": "number"
  },
  "items": [
    {
      "item_id": "string",
      "text": "string",
      "image": "string (path to image)",
      "actions": [
        {
          "type": "string",
          "payload": {
            "speed": "number"
          }
        }
      ]
    }
  ]
}
```

### Field Descriptions

- `meta.class_id`: Unique identifier for the lesson
- `meta.title`: Display name for the lesson
- `meta.date`: Creation date in YYYY-MM-DD format
- `meta.description`: Description of the lesson content
- `meta.seconds_per_word`: Duration each word is highlighted during training
- `items[].item_id`: Unique identifier for each flashcard
- `items[].text`: Text content to display
- `items[].image`: Path to the image file
- `items[].actions[].type`: Type of action (e.g., "flash")
- `items[].actions[].payload.speed`: Speed parameter for the action

## Development

### Prerequisites

- [Bun](https://bun.sh/) - JavaScript runtime and package manager
- [Rust](https://rust-lang.org/) - Systems programming language
- [Tauri CLI](https://tauri.app/) - Desktop app framework

### Setup

```bash
# Install dependencies
bun install

# Start development server
bun run dev

# Run Tauri in development mode
bun run tauri dev

# Build for production
bun run build
bun run tauri build
```

## Project Structure

```
flashbrain/
├── src/                    # Frontend (Svelte)
│   ├── lib/               # Shared components and utilities
│   ├── routes/            # SvelteKit routes
│   └── app.html          # Main HTML template
├── src-tauri/             # Backend (Rust)
│   ├── src/              # Rust source code
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── static/                # Static assets
│   └── classes/          # Lesson data
└── package.json          # Frontend dependencies
```

## License

MIT

## Description

This is a memorization reinforcement app designed to help structure memory implantation with
mnemomic presentation.  It reads from a JSON library of facts with the mnemoic images stored in
folder next to the json.  

# Vision-to-Language Learning App

**Tech Stack:** Svelte (Frontend), Rust (Backend), Tauri (App Shell)

## 📘 Purpose

A desktop application that visually introduces learners to content by first showing an image, then fading into associated text. This approach enhances memory by linking visuals with words. An optional feature highlights words one-by-one during display.

---

## 🎯 Goals

### Core Goals
- Load lessons from a JSON file containing image + text pairs
- Display the image fullscreen
- Fade image into the associated text
- Fine controls over fade and flash speed up to 11 unique frames per second
- Allow user to replay, skip, or load a new lesson

### Stretch Goals
- Highlight each word sequentially as the image fades
- Custom per-word timing via JSON
- Optional voice narration synced with highlighting (future)
- Custom theming and font support

---

## 📄 JSON Format

```json
{
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
            "speed": 11
          }
        }
      ]
    },
    {
      "item_id": "2",
      "text": "Does the sun undertake to do the work of the rain, or Aesculpius the work of the Fruit-bearer(the earth)? And how is it with respect to each of the stars, are they not different and yet they work together to the same end?",
      "image": "/static/classes/test-1/test_pattern.png",
      "actions": [
        {
          "type": "flash",
          "payload": {
            "speed": 10
          }
        }
      ]
    }
  ]
}
```

### Schema Details

#### Meta Object
- `class_id`: Unique identifier for the learning class/session
- `title`: Display name for the learning session
- `date`: Creation or last modified date (YYYY-MM-DD format)
- `description`: Detailed description of the learning content
- `seconds_per_word`: Time in seconds to display each word of text (e.g., 0.5 = half a second per word)

#### Items Array
Each item represents a single learning card:
- `item_id`: Unique identifier for the learning item
- `text`: The text content to display
- `image`: Path to the associated image file
- `actions`: Array of actions to perform with the item
  - `type`: Action type (e.g., "flash")
  - `payload`: Action-specific parameters
    - `speed`: Speed setting (1-11 frames per second)

### File Structure
```
static/classes/
├── session-name-1/
│   ├── training.json
│   └── image1.png
├── session-name-2/
│   ├── training.json
│   └── image2.png
└── ...
```

---

## 🧭 UI/UX Flow

1. **Startup**
   - Load default or user-selected JSON
2. **Lesson Display**
   - Show image fullscreen
   - Wait 2 seconds (configurable)
   - Fade out image over 2 seconds
   - Fade in text
   - *(Optional)* highlight each word using timing
3. **Controls**
   - Next / Previous lesson
   - Replay current lesson
   - Load new JSON
   - Toggle highlighting

---

## ⚙ Functional Requirements

| Feature                  | Description                                 |
|--------------------------|---------------------------------------------|
| JSON ingestion           | Load structured data from local disk        |
| Image rendering          | Display and fade effect                     |
| Text display             | Overlay on image or post-fade               |
| Word highlighting        | Animate focus word-by-word (optional)       |
| File picker              | Load new JSON datasets                      |
| Replay button            | Restart the current lesson display          |
| Configurability          | Duration of image fade and word intervals   |

---

## 🛠 Technical Architecture

### Frontend (Svelte)
- Image/text transition logic
- UI controls and event flow
- Word timing and highlight rendering
- Theme support and reactive state

### Backend (Rust via Tauri)
- JSON reading and schema validation
- File system access and image path resolution
- Persistent settings store (optional)

---

## ✨ Stretch Features

| Feature               | Description                                               |
|-----------------------|-----------------------------------------------------------|
| Per-word timing       | Words highlighted according to timing array               |
| Audio narration       | Sync TTS with text display and highlighting               |
| Word-focus mode       | Display one word at a time in center                      |
| Custom themes         | Dark/light mode, font selection, color themes             |
| Progress tracking     | Track which lessons have been viewed and how often        |

---

## 👤 User Stories

### Core
- *As a learner*, I want to see a picture before reading, to help memory.
- *As a user*, I want to replay the current content if I missed something.
- *As a user*, I want to load my own lesson sets via JSON files.

### Stretch
- *As a user*, I want each word highlighted to help focus on reading.
- *As an educator*, I want to control timing for different reading levels.

---

## 🧱 Non-Functional Requirements

- Cross-platform: Windows, macOS, Linux
- Launch time < 2s
- Memory usage < 100MB
- Responsive design
- Async loading (no UI freeze)

---

## ⚠ Constraints & Risks

- Font and layout consistency across OSes
- Large image files may delay transition
- Highlight sync needs to avoid layout jitter

---

## 📅 Milestones

| Milestone                         | Target Date |
|----------------------------------|-------------|
| JSON ingestion + basic UI        | Week 1      |
| Image fade + text overlay        | Week 2      |
| Word highlighting (stretch)      | Week 3      |
| Theming & polish                 | Week 4      |

---

## 📂 Future Ideas
- Sync with TTS or voiceover
- Export user stats on lessons viewed
- Cloud sync for shared lesson plans


# Appendix

## Initial Install Setup

```bash

% tauri-make flashbrain
✔ Identifier · com.----.flashbrain
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
✔ Choose your package manager · bun
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · JavaScript

Template created! To get started run:
  cd flashbrain
  bun install
  bun run tauri android init
  bun run tauri ios init

For Desktop development, run:
  bun run tauri dev

For Android development, run:
  bun run tauri android dev

For iOS development, run:
  bun run tauri ios dev
```
