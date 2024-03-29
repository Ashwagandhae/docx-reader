# Docx Reader v0.0.0

This is desktop app built with [Tauri](https://tauri.studio) and [Svelte](https://svelte.dev). You can open .docx files in it. Although many features are missing, like font, color, editing, etc, it opens documents much faster than word (~3x in dev server, ~28x faster after build). This means it can open a 4 MB Kritique Masterfile in ~1 second.

# Features

- Bold, underline, highlight, font size
- Outline
- Blazingly fast startup
- Zoom
- Search
- Multi-window support

# Unfeatures

### Features it doesn't have

- Ability to edit
- Pages
- Font, color, highlight color

# Download

## MacOS

1. [Click here](<https://ashwagandhae.github.io/docx-reader/Docx Reader_0.1.0_x64.dmg>) to download the dmg
2. Double click the dmg
3. Drag the app into Applications folder
4. Right click and click open (it won't let you open otherwise)

## Windows & Linux

Not supported right now (even though Tauri is literally cross platform)

# Or build from source

This will work for Macos, Windows and Linux

1. [Set up Tauri dependencies](https://tauri.studio/v1/guides/getting-started/prerequisites/)
2. Clone this repository and install packages

```
git clone https://github.com/Ashwagandhae/docx-reader
cd docx-reader
npm install
```

3. Start the development server (will be slow the first time because of rust packages)

```
npm run tauri dev
```

3. Or actually build the app (will be slow all the time)

```
npm run tauri build
```

# Dependencies explained

## Why Tauri?

[Tauri](https://tauri.studio) is basically a newer [Electron](https://www.electronjs.org/), but with much better performance. The fact that it uses [Rust](https://www.rust-lang.org/) in the backend allows it to parse the docx files fast.

## Why Svelte?

My Vue is that [Svelte](https://svelte.dev) is more Reactive, and it's easier to Express my ideas. I can see the Angle of it being a pretty new framework, but I think it's the Next big thing, and will probably become the Backbone of web development.

## Why quick_xml?

[quick_xml](https://docs.rs/quick-xml/latest/quick_xml/) is the fastest rust XML parsing library I could find. Instead of making a struct representation of the XML, it streams them as events. Docx files are actually just zipped up folders of XML files, you can [read more here](https://www.toptal.com/xml/an-informal-introduction-to-docx).

## Why zip?

[zip](https://docs.rs/zip/latest/zip/) is the first unzipping library I found. Docx files are actually just zipped up folders of XML files, so I needed to unzip them.
