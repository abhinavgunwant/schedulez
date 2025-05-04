# schedulEZ

Easy Scheduling.

Schedule shifts and various tasks in a few clicks.

## How does it work?

### Pre-requisite
Download the appropriate binary from the [releases](https://github.com/abhinavgunwant/schedulez/releases) page. Or:

- [Windows Binary](https://github.com/abhinavgunwant/schedulez/releases/download/v0.1.0/schedulez-win-x64.zip)

### Steps
- Create an input excel file. It just needs to have these three columns with any number of rows:
  - Name
    - This is the name of the element you want to schedule. e.g. employee name.
  - Prefer Days
    - Prefer these days
    - **Note:** Not supported as of now, leave this empty.
  - Avoid Days
    - The days to avoid for this element.
Alternatively, you can download the [input template excel](https://github.com/abhinavgunwant/schedulez/releases/download/v0.1.0/schedulEz-input-template.xlsx) from the releases page.
- Run the binary file
- Click on "Choose file" from the window that appears.
- Select the input file that you created.
- Click on "Generate Schedule".
- After the schedule is generated you'll see ".xlsx" button, click it.
- A file selection dialog will appear, select the ouput file.

## Building and running

Make sure you've installed the rust toolchain.

To build the release version:
```bash
cargo build --release
```

To run the debug version:
```bash
cargo run
```

