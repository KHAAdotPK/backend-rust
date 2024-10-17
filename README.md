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

#### License
This project is governed by a license, the details of which can be located in the accompanying file named 'LICENSE.' Please refer to this file for comprehensive information.

