### Chatbot Server, written in Rust
---

This repository contains the configuration and setup for a lightweight web server that will be used by chatbots currently under development. 

#### Purpose
The server will be used as the backend for chatbots that are being trained using advanced machine learning techniques. These chatbots are built using models trained on natural language data with:
- **Skip-gram**: A model that helps the chatbot understand the context of words in a sentence by predicting surrounding words.
- **CBOW (Continuous Bag of Words)**: A model that predicts a word based on its surrounding context, helping improve the chatbot's conversational ability.

#### Structure
The project is divided into several key modules:
1. `dict.rs`: 
   - Defines the structure and behavior of a dictionary (`Dict`).
   - Functions to parse content, update dictionary entries, and extract header and body information from HTTP requests.   - 
2. `content.rs`:
   - Handles content structures like headers and bodies extracted from the incoming streams.
3. `read_write.rs`:
   - Provides utilities for reading and writing content from and to the network stream.
4. `constants.rs`:
   - Stores constant values for delimiters, default file paths, and response codes used throughout the project.
5. `sundry.rs`
   - Handles connection, request and response details.

#### Foreign Function Interface (FFI) & Bindings
This Rust server also includes **Rust-C bindings**, enabling the Rust code to interact with functions written in C. Through this setup, C functions can call C++ code located in dynamically linked libraries (DLL files), creating a robust foreign function interface (FFI) across multiple languages. This allows the server to leverage existing C/C++ libraries and functionalities directly within the Rust environment.

#### Template Status
This project is a **working template** of the actual chatbot server. While it is functional, it serves as a testing ground for various technologies being integrated, such as machine learning models and foreign function interfaces (FFI). The project is continuously evolving as new techniques and technologies are tested.

#### Compilation Instructions
To compile this project, follow these steps:

1. **Run the Build Script**:
   - Execute the `BUILD.cmd` script located in the root directory of the project. This script will:
     - Navigate to the `src/native` directory and run the nested `BUILD.cmd` script to compile the native components.
     - Return to the root directory and run `cargo build` to compile the Rust components.
   ```bash
   ./BUILD.cmd
   ```
2. **Verify the Build**:
    - If the build is successful, you will see the message: `Cargo build completed successfully`.
    - If the build fails, the script will output an error message with the exit code.
3. **Output Files**:
    - The compiled native library (`clap.dll` and `clap.lib`) will be copied to the `target/<Configuration>` directory (e.g., `target/Debug`).
    - The Rust binaries will be generated in the `target/debug` or `target/release` directory, depending on the build configuration.
4. **Requirements**:
    - Ensure you have the necessary build tools installed, including:
        - MSBuild (part of `Visual Studio build tools`).
        - Rust toolchain (`cargo`).


#### License
This project is governed by a license, the details of which can be located in the accompanying file named 'LICENSE.' Please refer to this file for comprehensive information.

