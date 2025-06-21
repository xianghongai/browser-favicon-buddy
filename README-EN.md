# browser-favicon-buddy

A browser bookmark favicon loader.

## Features

- Batch-load website favicons contained in a bookmark file
- Operate through a graphical user interface
- Cross-platform: Linux, Windows and macOS

## Usage

Follow the steps below:

1. Export the *bookmark file* (`.html`) from your browser. In Firefox/Chrome you can press `Ctrl/Cmd + Shift + O` to open the Bookmark Manager.
2. Launch **favicon-buddy**, load the bookmark file, click **Start Processing** and wait for it to finish.
3. After completion a processed file will be generated in the same directory as the original, named `<original-filename>-with-favicons--<timestamp>.html`.
4. Import the generated bookmark file back into your browser.

## Notes

Chrome bookmark import:

- It can be *very* slow and unresponsive — please be patient.

Firefox bookmark import:

- The **Bookmarks Toolbar** will be imported into **Bookmarks Menu**. Select all entries and move them back to **Bookmarks Toolbar** if desired.
- Items that originally lived in **Bookmarks Menu** will be appended, which may cause stored *Keywords* to be lost.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
