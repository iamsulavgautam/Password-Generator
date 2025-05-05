# Password Generator API

A robust password generation service built with Rust and Actix-web that provides secure and customizable password generation capabilities.

## Features

- Customizable password length
- Optional numeric characters
- Optional special symbols
- Always includes both uppercase and lowercase letters
- RESTful API endpoints
- Secure random generation using `rand` crate
- Concurrent password generation using Rust's threading

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix-web
- **Random Generation**: rand crate
- **Serialization**: Serde
- **Logging**: env_logger, tracing

## API Endpoints

### 1. Default Password Generation

```
GET /
```

Generates a password with default settings:

- Length: 30 characters
- Numbers: Included
- Symbols: Included

### 2. Customized Password Generation

```
GET /config?length={length}&put_numbers={boolean}&put_symbols={boolean}
```

Query Parameters:

- `length` (optional): Integer value for password length (default: 16)
- `put_numbers` (optional): Boolean to include numbers (default: false)
- `put_symbols` (optional): Boolean to include symbols (default: false)

## Project Structure

```
password_generator-backend/
├── src/
│   ├── main.rs           # Application entry point
│   ├── api/
│   │   ├── handlers.rs   # Request handlers
│   │   ├── models.rs     # Data structures
│   │   ├── routes.rs     # API route definitions
│   │   └── mod.rs        # API module definitions
│   └── utils/
│       ├── generate_password.rs  # Password generation logic
│       └── mod.rs               # Utils module definitions
└── Cargo.toml            # Project dependencies and metadata
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd password-generator
```

2. Build the project:

```bash
cargo build
```

3. Run the server:

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## Implementation Details

### Password Generation

The password generation implementation includes:

- Thread-safe random number generation
- Concurrent character selection process
- Multiple character sets (lowercase, uppercase, numbers, symbols)
- Secure random selection using `rand` crate

### API Architecture

- RESTful design principles
- Query parameter validation
- JSON response format
- Logging middleware for request tracking
- Error handling and response formatting

## Security Features

- Cryptographically secure random number generation
- No password storage
- Request logging for security monitoring
- Input validation and sanitization

## Performance

The application utilizes Rust's concurrent features:

- Multi-threaded password generation
- Async web server implementation
- Efficient memory usage with Arc and Mutex

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

Created and maintained by RenaMice.
