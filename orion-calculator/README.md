# Orion Calculator

![Orion Calculator Screenshot](./Images/Calculator%20.png)

A futuristic, cross-platform scientific calculator built with Rust and Tauri.

---

## Table of Contents

- [Introduction](#introduction)
- [Core Features](#core-features)
- [Tech Stack & Architecture](#tech-stack--architecture)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running in Development](#running-in-development)
  - [Running Tests](#running-tests)
  - [Building for Production](#building-for-production)

---

## Introduction

Orion Calculator is a high-performance, cross-platform scientific calculator with a visually distinct "futuristic" aesthetic. It is built using Rust for the backend logic and web technologies for the frontend, all packaged within the Tauri framework.

## Core Features

- **Basic Arithmetic Operations:** Addition, subtraction, multiplication, and division.
- **Scientific Functions:** Parentheses and percentage.
- **Responsive & Intuitive UI:** A clean and modern user interface with a futuristic design.

## Tech Stack & Architecture

### Tech Stack

- **Core Logic:** [Rust](https://www.rust-lang.org/)
- **UI Framework:** [Tauri](https://tauri.app/)
- **UI Technology:** HTML5, CSS3, Vanilla JavaScript (ES6+)

### Architecture

This project adheres to the **Separation of Concerns (SoC)** principle. The backend business logic (the calculation engine in Rust) is completely decoupled from the frontend UI (the web view). Tauri facilitates this by design, allowing for a robust and maintainable codebase.

---

## Project Structure

```
/orion-calculator/
|-- /src/                   <-- Frontend Code
|   |-- index.html          (UI Structure)
|   |-- main.js             (UI Logic & Backend Calls)
|   |-- styles.css          (Futuristic Styling)
|
|-- /src-tauri/             <-- Backend Rust Code
|   |-- /src/
|   |   |-- main.rs         (Tauri setup & backend commands)
|   |   |-- engine.rs       (Core calculation logic)
|   |-- Cargo.toml          (Rust dependencies)
|   |-- tauri.conf.json     (Application configuration)
|
|-- /tests/                 <-- Rust unit tests
    |-- engine_tests.rs
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)

### Installation

1.  Clone the repository:
    ```sh
    git clone https://github.com/your-username/orion-calculator.git
    ```
2.  Navigate to the project directory:
    ```sh
    cd orion-calculator
    ```
3.  Install the frontend dependencies:
    ```sh
    npm install
    ```

### Running in Development

To run the application in development mode, use the following command:

```sh
npm run tauri dev
```

### Running Tests

To run the Rust unit tests for the calculation engine, navigate to the `src-tauri` directory and run:

```sh
cargo test
```

### Building for Production

To build the application for production, use the following command:

```sh
npm run tauri build
