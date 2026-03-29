<div align="center">

<img width="100%" src="https://capsule-render.vercel.app/api?type=waving&color=ffffff&height=200&section=header&text=Focus%20Flow&fontSize=50&fontColor=000&animation=twinkling&fontAlignY=40&desc=A%20Pomodoro%20app%20to%20maximize%20your%20productivity.&descAlignY=60&descSize=18">

<p align="center">
  <i>A minimalist and elegant Pomodoro timer with ambient sounds, task management, and productivity tracking, rebuilt from the ground up using Rust and Leptos for high-performance Client-Side Rendering (CSR).</i>
</p>

---

### Features

<div align="center">

| Feature | Description |
|:---:|:---|
| Design | Monastic minimalist design strictly driven by custom CSS |
| Native Speed | High-performance logic rendering natively via WebAssembly (WASM) |
| Task management | In-memory CRUD task management (Add, complete, delete) |
| Active Focusing | Interactive selection of active projects targeting deep work |
| Custom intervals | Highly customizable intervals for Focus and Breaks via clean UI Modals |
| Local Storage | Zero-database architecture, entirely persisted on Browser Cache (`gloo-storage`) |
| SPA Navigation | Fluid routing toggling without page reloading via Global State Context |

</div>

### Getting Started

To run this project locally, you'll need the Rust compiler, target architectures for web compiling, and `trunk` as the bundler.

```bash
# Clone the repository
git clone https://github.com/matheussricardoo/FocusFlow.git

# Navigate to project directory
cd FocusFlow

# Install Rust WASM architecture
rustup target add wasm32-unknown-unknown

# Download Trunk (our web builder pipeline)
cargo install trunk

# Run the development server with live reloading
trunk serve --open
```

### Technologies

This completely rebuilt environment shifts away from typical Node.js loops to offer lighting-fast memory security directly inside your browser.

<div align="center">

<a href="https://www.rust-lang.org/"><img src="https://skillicons.dev/icons?i=rust" alt="Rust"/></a>
<a href="https://webassembly.org/"><img src="https://skillicons.dev/icons?i=wasm" alt="WebAssembly"/></a>
<a href="https://developer.mozilla.org/en-US/docs/Web/CSS"><img src="https://skillicons.dev/icons?i=css" alt="Vanilla CSS"/></a>
<a href="https://github.com/features/actions"><img src="https://skillicons.dev/icons?i=githubactions" alt="GitHub Actions"/></a>

*Framework: Leptos (Reactive CSR Framework for Rust)*

</div>

### Project Structure

Following standard Rust Client-Side architectural parameters using Trunk.

```text
FocusFlow/
├── src/
│   ├── components/
│   │   ├── projects.rs      # Projects UI, State arrays, and CRUD logic
│   │   ├── settings.rs      # Modal for custom intervals
│   │   ├── stats.rs         # Analytics views and context footers
│   │   └── timer.rs         # Pomodoro recursive reactive interval logic
│   ├── app.rs               # Context Provider, Routing, and LocalStorage Hook
│   └── main.rs              # Rust insertion pointer point
├── style/
│   └── main.css             # Vanilla CSS controlling all Minimalist Layouts
├── .github/workflows/
│   └── deploy.yml           # Automated deployment pipeline to GitHub Pages
├── Cargo.toml               # Rust dependencies (Leptos, Serde, Gloo Storage)
├── index.html               # Main HTML entry frame
└── favicon.svg              # Clean SVG graphic logo 
```

### Author

<div align="center">
  <a href="https://github.com/matheussricardoo" target="_blank">
    <img src="https://skillicons.dev/icons?i=github" alt="GitHub"/>
  </a>
  <a href="https://www.linkedin.com/in/matheus-ricardo-426452266/" target="_blank">
    <img src="https://skillicons.dev/icons?i=linkedin" alt="LinkedIn"/>
  </a>
</div>

### License

This project is licensed under the MIT License.

<img width="100%" src="https://capsule-render.vercel.app/api?type=waving&color=ffffff&height=120&section=footer"/>

</div>
