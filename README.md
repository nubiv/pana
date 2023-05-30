# Lobot 🤖

This is a desktop application that allows users to ask questions to their documents without requiring an internet connection, using the power of LLMs. The app ensures complete privacy as no data leaves the user's execution environment at any point.

## Technologies Used

- Tauri
- SvelteKit
<!-- - Tailwind CSS -->

## Prerequisites

Before getting started with the Tauri project, ensure that you have the following prerequisites installed on your machine:

1. Node.js: Make sure you have Node.js installed. You can download it from the official website: https://nodejs.org.

2. Rust: Tauri is powered by Rust, so you'll need to have Rust installed. You can install Rust by following the instructions on their website: https://www.rust-lang.org.

Once you have these prerequisites installed, you're ready to start working with Tauri and building your applications.

## Getting Started

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Install dependencies using your preferred package manager, e.g. `npm install` or `yarn install`.

## Development

To start the development server, run `npm run tauri dev`, or `cargo tauri dev` if you have Tauri CLI installed. This will launch the Tauri application and open a development window. Any changes you make to the source code will automatically reload the application.

## Download

Not yet.

# Roadmap

Offline Mode

- [x] Uncontextual conversation with LLM model (currently only tested with Llama model)
- [ ] Contextual conversation with LLM model for assistance and support
- [ ] Search functionality to quickly locate information within the document
- [ ] Summarization of lengthy documents for easy understanding

Online Mode (Not sure if this is needed)

- [ ] Interaction with ChatGPT model using your own key

## Contributing

Contributions are always welcome! Please submit a pull request if you'd like to contribute to this project.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
