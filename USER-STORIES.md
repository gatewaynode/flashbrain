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

* The langing page has the required navigation items.
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