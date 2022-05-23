# Docx reader

This is desktop app built with [Tauri](https://tauri.studio) and [Svelte](https://svelte.dev). You can open .docx files in it. Although many features are missing, like font, color, editing, etc, it opens documents ~3x faster than word.

## Features

- Bold, underline, highlighting, font size
- Outline

## Download

### MacOS

1. [Click here](https://ashwagandhae.github.io/docx-reader/docx-reader.app.zip) to download a zip
2. Unzip the zip file (I swear its not a virus)
3. Double click the app

### Windows & Linux

Not supported right now (even though Tauri is literally cross platform)

## Or build from source

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

## Why Tauri?

Tauri is basically a less developed [Electron](https://www.electronjs.org/), but with much better performance. The fact that it uses [Rust](https://www.rust-lang.org/) in the backend allows it to parse the docx files fast.

## Why Svelte?

My point of Vue is that Svelte is more Reactive, and it's easier to Express my ideas. I can see the Angle of it being a pretty new framework, but I think it's the Next big thing, and will probably become the Backbone of web development.
