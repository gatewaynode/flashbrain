# Agile Project Log

Because of course we want to be agile and work through a series of stories as we build our app.

## Setup phase

For this I was pretty straightforwad and brief and actually asked Cursor to write it's own Cursor project rules with this prompt.

    This is a Tauri app made up of Javascript and Rust parts with a Bun webserver, so let's make some Cursor project rules .mdc files in .cursor/rules that represent best practices in Javascript(no typescript) and Rust. 

    Some of my preferences that I want to catpure in the rules are staying on the latest stable dependencies, keeping code short and simple, making sure to explain code  with comments, and working on small modular pieces of logic.

I had to rename the files from .md to .mdc, but everything else seemed ok in review.

## MVP Phase

I'll create user stories as prompts as we go, each story except the first will be on its own git branch and require a PR before merging it in.  As part of the review I'm going to make sure the app launches locally, the features I expect to see are there, and once we get to adding in tests that they all pass.

### Story 1

"As a user I should be presented with a landing page with the menu options of 'new', 'resume', 'find', 'create', 'settings'.  The menu options all go to placeholder pages, but should otherwise be functional.  Use the website '@https://fiveyears.minus99.com/ ' as inspiration for the design elements and visuals.

Acceptance criteria:

* The landing page has the required navigation items.
* The navigation items go to similarly styled stub pages."

### Story 2

"As a developer on this application I want to make sure we are not using any deprecated javascript or browser functions.  Other developers have mentioned the following warning:

Using `on:click` to listen to the click event is deprecated. Use the event attribute `onclick` instead

Let's move the code to the onlcick attribute before we procede.

Acceptance criteria:

* The `on:click` event handler is replaced with `onclick` attribute
"

### Story 3

"As a user the app window is opening too short to show the full landing menu.

Adjust the default app height by about 40% to display the entire menu and footer.

Acceptance criteria:

* The Tauri app opens about 40% taller so all menu items and the footer are visible."

### Story 4

"As a user when I click on new session the page will give me a list of available learning paths.

The list items can be filled from the json files inside each directory in the static/classes folder.  Each JSON contains a `meta` field with the title, date and description that should be used in the display list.

Also update the README.md format to match the JSON schema in static/classes/test-1/training.json.

Acceptance criteria:

* The New Session page displays a list of available sessions derived from the JSON meta field
* The title, date and description are displayed in the list
* The README JSON schema matches what is in the provided static/classes/test-1/training.json file"

### Story 5

"As a user when I click on the 'Start Learning' button on the 'New Session' the JSON should be loaded in a global state variable called current_training_data and I should be taken to a new screen that iterates through the items.  The items present as their image fitted for the screen and then the image fades out as the text fades in.

When all items are iterated through, the process starts over looping through the JSON 'items' list until the back button is clicked.

Also update the README.md JSON schema to include the new 'class_id' and 'item_id' fields from static/classes/test-1/training.json.

Acceptance criteria:

* Clicking an item listed in new session loads the JSON item described
* Clicking an item also opens up the training page where the items are displayed
* The items are displayed as image first and then faded, while the text is faded in at the same time"


### Story 6


"The user needs to see a more refined animation experience, and we need to make this more modular.

The image should only fade to 30% transparency.

The text background should only fade in to 70% transparency.

The javascript part of the item display should be made into a reusable module in the src/lib directory.

Acceptance criteria:

* The fade is a crossfade that stops at 30% for the image and 70% for the text background.
* The JSON retrieval and looping logic should be made as a reusable module.
"

### Story 7

"As a user the create section should be where I go to add new content which is also the place to edit existing content.

The options on that page should be create, edit, or import.

The `create` option should open a dialogue modal for a new lesson name, which will be the directory name of the lesson in the manipulation page.

The `edit`  option should open a file picker widget to get the file path for the manipulation page.

The `import` option should be stubbed out but non-functional.

A new editor page should be stubbed out for lesson manipulation.

The settings page needs a new option for the lessons file path(a global variable) to save the lesson dirs to, by default it should be an XDG path to the documents directory under a folder called flashbrain.

Acceptance criteria:
* The options of create, edit and import have been added to the create page.
* A editor page has been stubbed out that captures the filepath from the create and edit options,
* The settings page now has a option for setting the global path for saving lesson dirs.
"

### Story 8

"As a user I should see the appropriate file dialogues appear on MacOS.  

Use the example code at https://github.com/gatewaynode/file-dialogue-popper to see how this can be done with the RFD crate on the Rust side of the app.

Change the 'create-->edit existing lesson' and 'create-->import from file' to use Rust with the RFD crate to open the file dialogues and return JSON even if the dialogue modals are cancelled similarly to 'file-dialogue-popper'.

Acceptance criteria:

* The edit and import functions under 'Create' open OS file dialogues.
* The operation always returns JSON results that are logged server side.

**COMPLETED**: 
- Added RFD crate dependency to Cargo.toml
- Implemented open_file_dialog and save_file_dialog Tauri commands
- Updated frontend to use Rust file dialogues instead of Tauri dialog plugin
- Added comprehensive error handling and JSON result logging
- Created unit tests for file dialogue functionality
- Added documentation in FILE_DIALOGUE_IMPLEMENTATION.md
"

### Story 9

"As a user using the training feature I should see the word that corresponds to each measure of time in extra bold font.  So that the words are bolded and increased in size 30% in order from first to last in sync with the display duration of the text.

Acceptance criteria:

* All words get enhanced when it is their part of the duration during training.

**COMPLETED**: 
- Implemented word-by-word highlighting in ItemDisplay component
- Added word splitting functionality to utils.js
- Words become bold (font-weight: 700) when highlighted
- Highlighting is synchronized with text display duration
- Added smooth transitions and visual effects (text-shadow)
- Enhanced utility functions for word processing and timing calculations
- Added comprehensive logging for debugging word highlighting sequence
- **UPDATED**: Removed font size increase to prevent layout jitter, keeping only bold font weight
"

### Story 10

"As a user opening a file to edit should take me to the visual editor pane with the file parsed into nodes starting with the metadata node and each item from the list in it's own node.  The fields from the file should be visible in the nodes and each is an editable field.  The image path should be a text field with an icon to open the file dialogue to pick an image from a the OS file dialogue if I don't know the path.

- Use Svelte-Flow for the editor nodes from https://svelteflow.dev/
- The metadata section must be the parent node
- The first item node is a child of the metadata section node
- All subsequent items are children of the previous item
- A mock tools panel is on the left hand sidebar of the editor
- Standard menu options are mocked in the top bar of the editor pane

Acceptance criteria:

* Opening a file for edit opens the editor page with JSON loaded in Svelteflow nodes
* The fields are all exposed as editable
* The image field contains an icon to open the OS file dialogue
* Other tool and menu options are mocked out

**COMPLETED**:
- Installed @xyflow/svelte package for Svelte-Flow integration
- Created MetaNode.svelte component for metadata editing with all fields (class_id, title, date, description, seconds_per_word)
- Created ItemNode.svelte component for item editing with all fields (item_id, text, image path, speed)
- Implemented visual editor page with Svelte-Flow canvas, background, controls, and minimap
- Added proper node hierarchy: metadata as parent, items as children connected sequentially
- Created left sidebar tools panel with mock buttons (Add Item, Delete Item, Duplicate Item, Import Image, Export JSON)
- Added top menu bar with mock menu options (File, Edit, View, Help, Save)
- Implemented JSON loading from file path with error handling and loading states
- Added image file dialog button (📁) in item nodes for future OS file dialogue integration
- All form fields are editable with proper event handling and data updates
- Added comprehensive styling matching the app's dark theme with glassmorphism effects
- Verified successful build and integration with existing Tauri backend
- **UPDATED**: Increased app window size (width: 1600px, height: 1224px) for better editor experience
- **UPDATED**: Changed editor canvas background to pure black for better contrast
- **UPDATED**: Fixed node spacing with 300px vertical distance between nodes for better readability
- **UPDATED**: Improved node positioning with metadata at x:400, y:100 and items starting at y:350
- **UPDATED**: Fixed edge connections by adding proper source/target handles to MetaNode and ItemNode components
- **UPDATED**: Added comprehensive logging and error handling for edge creation debugging
- **UPDATED**: Added Handle components with unique IDs for proper React Flow edge connections
- **UPDATED**: Added onError callback to SvelteFlow for better error reporting and debugging
- **UPDATED**: Created CustomEdge.svelte component with bright red (#ff0000), animated, and thicker (4px) styling
- **UPDATED**: Added animated dash pattern (10,5) with continuous animation for better visual flow
- **UPDATED**: Registered custom edge type in SvelteFlow for enhanced edge visibility and user experience

#### Code Cleanup

##### Duration Field Removal

Removed the unused `duration` field from the training JSON schema and associated code.

**COMPLETED**:
- Removed `duration` field from `static/classes/test-1/training.json`
- Removed `duration` field from `ActionPayload` struct in Rust backend
- Updated README.md JSON schema documentation to remove duration field
- Verified both frontend and backend build successfully
- The `seconds_per_word` field in meta section is used for word highlighting timing

##### Lesson Detection Enhancement

Enhanced the lesson detection system to support both `training.json` and `lesson.json` file formats and added comprehensive testing.

**COMPLETED**:
- Updated `get_learning_paths()` function to detect both `training.json` and `lesson.json` files
- Updated `load_training_data()` function to support both file formats
- Added comprehensive logging to track directory scanning and file detection
- Created `test_lesson_discrepancy()` function to detect missing or malformed lesson files
- Added frontend test button and results display on the New Session page
- Fixed issue where only 1 of 3 available lessons was being detected
- Added detailed discrepancy reporting showing:
  - Total directories found
  - Directories with JSON files (training.json vs lesson.json)
  - Directories without JSON files
  - Successful vs failed JSON parses
  - Expected vs actual lesson counts
- Verified all 3 lessons now appear in the New Session page
- **UPDATED**: Hidden test UI behind debug flag from settings page for cleaner production interface

### Story 10

"As a user opening a file to edit should take me to the visual editor pane with the file parsed into nodes starting with the metadata node and each item from the list in it's own node.  The fields from the file should be visible in the nodes and each is an editable field.  The image path should be a text field with an icon to open the file dialogue to pick an image from a the OS file dialogue if I don't know the path.

- Use Svelte-Flow for the editor nodes from https://svelteflow.dev/
- The metadata section must be the parent node
- The first item node is a child of the metadata section node
- All subsequent items are children of the previous item
- A mock tools panel is on the left hand sidebar of the editor
- Standard menu options are mocked in the top bar of the editor pane

Acceptance criteria:

* Opening a file for edit opens the editor page with JSON loaded in Svelteflow nodes
* The fields are all exposed as editable
* The image field contains an icon to open the OS file dialogue
* Other tool and menu options are mocked out

**COMPLETED**:
- Installed @xyflow/svelte package for Svelte-Flow integration
- Created MetaNode.svelte component for metadata editing with all fields (class_id, title, date, description, seconds_per_word)
- Created ItemNode.svelte component for item editing with all fields (item_id, text, image path, speed)
- Implemented visual editor page with Svelte-Flow canvas, background, controls, and minimap
- Added proper node hierarchy: metadata as parent, items as children connected sequentially
- Created left sidebar tools panel with mock buttons (Add Item, Delete Item, Duplicate Item, Import Image, Export JSON)
- Added top menu bar with mock menu options (File, Edit, View, Help, Save)
- Implemented JSON loading from file path with error handling and loading states
- Added image file dialog button (📁) in item nodes for future OS file dialogue integration
- All form fields are editable with proper event handling and data updates
- Added comprehensive styling matching the app's dark theme with glassmorphism effects
- Verified successful build and integration with existing Tauri backend
"

### Story 11

"As a user the edit page needs to hide the information in nodes to the bare minimum in the main edit pane, and then when I click on an expand icon the nodes open up to show all fields.  The layout also needs to be automatically arranged to show a tree view.

- Nodes only show the `item_id` and an expand icon to show the rest of the fields
- The library Dagre,https://github.com/dagrejs/dagre/wiki, should be used to provide a default SvelteFlow layout per https://svelteflow.dev/learn/layouting/layouting-libraries
- The node image field icon is there but the implementation is TODO, please complete this functionality per story 10.

**COMPLETED**:
- Added Dagre library dependency for automatic tree layout
- Updated MetaNode component to support collapsed/expanded states with expand/collapse icons
- Updated ItemNode component to support collapsed/expanded states with expand/collapse icons
- Implemented automatic tree layout using Dagre with top-to-bottom direction
- Added "Auto Layout" button in tools panel to manually trigger layout
- Completed image file dialog functionality in ItemNode component using RFD crate
- Nodes now show only item_id/class_id when collapsed, with expand icon to reveal all fields
- Automatic layout is applied when editor loads and can be manually triggered
- Tree view maintains parent-child relationships (metadata → first item → subsequent items)
- Image file dialog supports common image formats (png, jpg, jpeg, gif, bmp, webp)
- All functionality integrated with existing Tauri backend file dialog system
"

### Story 12

'As a user the training items we are going to need some additional fields.  We need to update the schema in the `lesson.json` to support better display and contextualization.  And we need to change the `training.json` to be a parent element for multiple `lesson.json` items.

- Add the string field `title` to each item in the `items` list for both `static/classes/rsti_lesson_package/lesson.json` and `static/classes/tcrei_lesson_package/lesson.json`
- Update the documentation and all implementations of the item variable including the Rust `pub struct TrainingItem` for the new item title
- Add the string field `acronym` to each item in the `items` list for both `static/classes/rsti_lesson_package/lesson.json` and `static/classes/tcrei_lesson_package/lesson.json`
- Update the documentation and all implementations of the item variable including the Rust `pub struct TrainingItem` for the new item acronym
- Change the string field `class_id` in the metadata section to `lesson_id` in both `static/classes/rsti_lesson_package/lesson.json` and `static/classes/tcrei_lesson_package/lesson.json`
- Update the documentation and all implementations of the item variable including the Rust `pub struct Meta` for the new item lesson_id
- Don't touch `static/classes/test-1/training.json` that's for the next story.

The new schema for lesson.json files should match this:
```json
{
  "meta": {
    "lesson_id": "string",
    "title": "string",
    "date": "string (YYYY-MM-DD)",
    "description": "string",
    "seconds_per_word": "number"
  },
  "items": [
    {
      "title": "string",
      "acronym": "string",
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

Acceptance criteria:

- The lesson.json file schema has been updated and all implementations of the code have been updated to match the new schema
- The documentation has been updated to reflect the changes to the schema

**COMPLETED**:
- Updated both lesson.json files (rsti_lesson_package and tcrei_lesson_package) with new schema
- Added `title` and `acronym` fields to each item in both lesson.json files
- Changed `class_id` to `lesson_id` in metadata section of both lesson.json files
- Updated Rust structs (`Meta` and `TrainingItem`) to match new schema
- Updated frontend components (MetaNode and ItemNode) to use new field names
- Updated editor page to use `lesson_id` instead of `class_id`
- Updated README.md documentation to reflect new schema
- Temporarily modified parsing logic to skip training.json files until Story 13
- Verified successful build and integration
'

### Story 13

'As a user the `training.json` file should be converted to a parent data type capable of linking to additional `lesson.json` files, so that multiple trainings can be strung together.

- Add the necessary Rust structs to match the following schema as a new complex data type
- Update the `static/classes/test-1/training.json` to match the following schema:

```json
{
  "meta": {
    "class_id": "string",
    "title": "string",
    "date": "string (YYYY-MM-DD)",
    "description": "string",
    "custom_order": "string"
  },
  "children": [
    {
      "lesson_id": "string",
      "default_order": "number",
      "title": "string",
      "gated": "boolean",
      "gates": [
        {
          "type": "string",
          "payload": {
            "speed": "number"
          }
        }
      ]
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

- Update the documentation to add the new data type
- The "New" page will now parse `training.json` as a multiple type and show slightly similar information but clearly indicate a multiple lesson format.

Acceptance criteria:

- A new parent data type is available to link multiple lessons together.