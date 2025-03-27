# dHisStoryGame.AI

> [!IMPORTANT]
> This project depends on `ic_llm`, but in version `0.3.0` it does not have **assistant** role. The `0.3.0` version should be marked as incomplete since it is critical to have **assistant** role for a fine-tuned context-aware LLM model. This project will use a workaround to handle the missing **assistant** role type of message from `ic_llm 0.3.0`. Learn more about the difference between system & assistant message here: [openAI forum](https://community.openai.com/t/what-exactly-does-a-system-msg-do/459409/2)

## How It Works

### Initial Prompt

When a guest clicks "Start", the frontend sends an initial prompt to the backend. This prompt combines a system message with an introductory narrative asking the user to describe themselves (real character or not, and the preferred location).

### Conversation Continuation (Original Design)

Each subsequent user message is combined with that user’s conversation history (retrieved by their Principal). If the history grows beyond a set threshold, a summarization routine is invoked to compress the conversation context and the token count. The full prompt (system message + history + new message) is then sent to the LLM.

### Conversation Continuation (Workaround `ic_llm v0.3.0`)

Each pair of user message and LLM's response is combined with that user's conversation history (retrieved by their Principal). If the history grows beyond a set threshold, a summarization routine is invoked to compress the conversation context and token count. The full prompt (system message combined with history + new message) is then sent to the LLM.

### LLM Response

The backend call to the ollama canister Llama 3.1:8B with the assembled prompt. The LLM’s response (which continues the narrative and provides a bullet list of new choices) is returned and appended to the user’s history.

### User-Specific Storage

Conversation histories are maintained in-memory per user using a global HashMap keyed by the user’s Principal. This ensures that each guest’s experience is isolated and personalized.

## Project structure

The `/backend` folder contains the Rust smart contract:

- `lib.rs`, which contains the smart contract and llm, and exports its interface

The `/frontend` folder contains web assets for the application's user interface. The user interface is written using the React framework.

---

## Getting Started

To build the project locally, follow these steps.

### 1. Clone the repository

```sh
git clone git@github.com:muhrifqii/ICV.git
```

### 2. Setting up Ollama

This project requires a running LLM model. To be able to test the agent locally, you'll need a server for processing the agent's prompts. For that, we'll use `ollama`, which is a tool that can download and serve LLMs.
See the documentation on the [Ollama website](https://ollama.com/) to install it. Once it's installed, run:

```
ollama serve
# Expected to start listening on port 11434
```

The above command will start the Ollama server, so that it can process requests by the agent. Additionally, and in a separate window, run the following command to download the LLM that will be used by the agent:

```
ollama run llama3.1:8b
```

The above command will download an 8B parameter model, which is around 4GiB. Once the command executes and the model is loaded, you can terminate it. You won't need to do this step again.

### 3. Install developer tools.

> Installing `dfx` natively is currently only supported on macOS and Linux systems. On Windows, you should run everything inside WSL, including the project itself.

> On Apple Silicon (e.g., Apple M1 chip), make sure you have Rosetta installed (`softwareupdate --install-rosetta`).

1. Install `dfx` with the following command:

   ```
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

1. [Install NodeJS](https://nodejs.org/en/download/package-manager) or use dev tools such as `nvm`.

1. Install [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo): `curl https://sh.rustup.rs -sSf | sh`

1. Install [candid-extractor](https://crates.io/crates/candid-extractor): `cargo install candid-extractor`

Lastly, navigate into the project's directory.

### 4. Create a local developer identity.

To manage the project's canisters, it is recommended that you create a local [developer identity](https://internetcomputer.org/docs/building-apps/getting-started/identities) rather than use the `dfx` default identity that is not stored securely.

To create a new identity, run the commands:

```
dfx start --background

dfx identity new IDENTITY_NAME

dfx identity use IDENTITY_NAME
```

Replace `IDENTITY_NAME` with your preferred identity name. The first command `dfx start --background` starts the local `dfx` processes, then `dfx identity new` will create a new identity and return your identity's seed phase. Be sure to save this in a safe, secure location.

The third command `dfx identity use` will tell `dfx` to use your new identity as the active identity. Any canister smart contracts created after running `dfx identity use` will be owned and controlled by the active identity.

Your identity will have a principal ID associated with it. Principal IDs are used to identify different entities on ICP, such as users and canisters.

[Learn more about ICP developer identities](https://internetcomputer.org/docs/building-apps/getting-started/identities).

### 5. Deploy the project locally.

Deploy your project to your local developer environment with the command:

```
dfx deploy
```

Your project will be hosted on your local machine. The local canister URLs for your project will be shown in the terminal window as output of the `dfx deploy` command. You can open these URLs in your web browser to view the local instance of your project.

---

## Acknowledgement

This project meant to be able to be deployed on to [icp.ninja](https://icp.ninja). At the time this project is written, there's a couple **constraint** from current [icp.ninja](https://icp.ninja) version:

- `ic_llm` on the base sample project is using version `0.3.0` which does not consit of `Role::Assistant`. Thus this project wont work as the original plan and used some workaround because of it, unless using the `0.4.0` or any latest version.
- `Cargo.toml` and `package.json` are both locked, preventing the project to adopt more crates/package that is not added from the base sample project.
- Folder cannot be added.
- `dfx.json` also locked, preventing the project to add more canister other than the base sample project used. in this case, the project cannot use **Internet Identity**.
- Rust file cannot be added.

Once any of those limitations are resolved on ICP ninja, we can also lift them from this project and make more enhancements.
