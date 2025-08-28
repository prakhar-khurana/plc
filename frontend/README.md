# PLC Secure Coding Practices Checker Frontend

This project provides a modern, responsive web interface for checking PLC source code against a set of secure coding practices. It is built with **React**, **TypeScript**, **Vite**, and **Tailwind CSS**, and it integrates with a Rust-based checker compiled to WebAssembly.

## Features

- **File Upload** – Drag‑and‑drop or click to select PLC files (`.scl`, `.st`, `.xml`, `.il`, `.awl`).
- **Custom Policy** – Optionally paste a JSON policy to override default rule severities or configurations.
- **Wasm Integration** – Calls the exported `check_plc_code` function from the Rust Wasm module to perform analysis entirely on the client.
- **Results Display** – Shows followed rules and detailed cards for any violations, including line number, reason, and suggestion.
- **Dark Theme** – Uses Tailwind CSS to provide a clean, dark UI that is fully responsive.

## Project Structure

```text
plc-frontend/
├── index.html              # Main HTML entry point
├── package.json            # Project metadata and dependencies
├── postcss.config.js       # PostCSS configuration
├── tailwind.config.js      # Tailwind CSS configuration
├── tsconfig.json           # TypeScript configuration
├── vite.config.ts          # Vite configuration
├── README.md               # This file
└── src/                    # Source code
    ├── main.tsx           # Application entry for React
    ├── index.css          # Global styles (Tailwind base)
    ├── App.tsx            # Main container component
    └── components/        # Reusable UI components
        ├── Header.tsx
        ├── FileInput.tsx
        ├── PolicyInput.tsx
        ├── Results.tsx
        └── ViolationCard.tsx
```

## Getting Started

1. **Install Dependencies**

   Navigate into the `plc-frontend` directory and install the dependencies:

   ```bash
   npm install
   ```

2. **Compile the Rust Backend to WebAssembly**

   The GUI depends on a Wasm module exposing a single function `check_plc_code`. Use [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) to build the Rust project (assumed to be in `plc-main/`):

   ```bash
   cd /path/to/plc-main
   wasm-pack build --release --target web
   ```

   This command generates a `pkg/` directory containing `plc_checker.js` and `plc_checker_bg.wasm`.

3. **Place the Wasm Files**

   Copy the generated Wasm package into the `wasm/` directory of the frontend (create the directory if it doesn’t exist). The frontend code dynamically imports `../wasm/plc_checker.js` relative to `src/App.tsx`.

   ```bash
   mkdir -p plc-frontend/wasm
   cp pkg/plc_checker.js pkg/plc_checker_bg.wasm plc-frontend/wasm/
   ```

4. **Start the Development Server**

   From within the `plc-frontend` directory, start the local dev server:

   ```bash
   npm run dev
   ```

   The application will be available at `http://localhost:5173` by default.

5. **Build for Production**

   To build a production bundle:

   ```bash
   npm run build
   ```

   Preview the production build locally with:

   ```bash
   npm run preview
   ```

## Important Notes

- **No CORS Issues** – Because the Wasm module is loaded locally from the same origin, there are no CORS or port‑related problems when calling `check_plc_code`. All data stays within the browser.
- **File Types** – Only `.scl`, `.st`, `.xml`, `.il`, and `.awl` files are accepted. Unsupported file types trigger an alert.
- **Error Handling** – Basic error messages are displayed if something goes wrong during analysis. Open the browser console for detailed errors.

Enjoy using the PLC Secure Coding Practices Checker!
