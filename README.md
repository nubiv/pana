# Lobot 🤖

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/Ryc9Cbws)
![License](https://img.shields.io/github/license/sobelio/llm-chain?style=for-the-badge)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

A locally hosted desktop assistant leveraging LLMs, enabling offline question-answering with complete data privacy.

> **🔔 Note!!!**  
> This project is currently in its early stages of development.

## Stacks Used

- [tauri](https://github.com/tauri-apps/tauri)
- [svelte-kit](https://github.com/sveltejs/kit)
- [tailwindcss](https://github.com/tailwindlabs/tailwindcss)

## Prerequisites

Before getting started with the Tauri project, ensure that you have the following prerequisites installed on your machine:

1. Node.js: Make sure you have Node.js installed. You can download it from the official website: https://nodejs.org.

2. Rust: Tauri is powered by Rust, so you'll need to have Rust installed. You can install Rust by following the instructions on their website: https://www.rust-lang.org.

<!-- 3. LLM: Place a GGML-targeting `.bin` LLM model (currently only tested with Llama model) in the `llm` folder. -->

Once you have these prerequisites installed, you're ready to start working with Tauri and building your applications.

## Getting Started

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Install dependencies using your preferred package manager, e.g. `npm install` or `yarn install`.

## Development

To start the development server, run `npm run tauri dev`, or `cargo tauri dev` if you have Tauri CLI installed. This will launch the Tauri application and open a development window. Any changes you make to the source code will automatically reload the application.

## Download

Not yet.

## Roadmap

<!-- Offline Mode -->

<!-- - [x] Uncontextual conversation with LLM model -->

- [ ] Contextual conversation with LLM model for assistance and support
- [ ] Search functionality to quickly locate information within the document
- [ ] Summarization of lengthy documents for easy understanding

<!-- Online Mode (Not sure if this is needed)

- [ ] Interaction with ChatGPT or other models using your own key -->

## Credits

- Thanks to the authors of [llm](https://github.com/rustformers/llm) and [llm-chain](https://github.com/sobelio/llm-chain) for the Rust bindings over [Llama.cpp](https://github.com/ggerganov/llama.cpp).
- Thanks to the authors of [shadcn-svelte](https://github.com/huntabyte/shadcn-svelte) for porting [shadcn-ui](https://github.com/shadcn/ui) to Svelte.

## Contributing

Contributions are always welcome! Please submit a pull request if you'd like to contribute to this project.

## License

`Lobot` is licensed under the [MIT License](LICENSE.md).

## Connect

If you have any questions, suggestions, or feedback, feel free to open an issue or join [discord](https://discord.gg/Ryc9Cbws).
