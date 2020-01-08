# Wasm/JS Interop example

This repository contains example code for a demo i did during a presentation.  
It shows interoperability between JS and WASM compiled from Rust. 

It utilizes the _wasm-bindgen_ crate to bind javascript functions in wasm and expse wasm functions to javascript. 



## How to use

### Basic setup
To get started you will need to setup a [rust toolchain](https://rustup.rs/).  
If you are using VS Code i would also recommend the __Rust (rls)__ extension.    


### Install dependencies
To install Javascript dependencies, simply run the `yarn` command from your cli in the project root.   
Rust dependencies should be installed automatically, if they are not run `cargo build`.  

### Development server
To run the development server, run `yarn serve`.  
You will be able to access it on [localhost:8080](localhost:8080) by default.  

### Build
You shouldn't need to build the project, if you wish to do so nevertheless, run `yarn build`
