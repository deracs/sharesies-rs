# Rust Shares API SDK

This project provides a Rust SDK for interacting with the Shares API. It includes functionality for authentication, fetching portfolio data, and managing tokens securely.

## Features

- Authentication using login credentials.
- Secure token storage.
- Fetch portfolio data.
- Modular and clean architecture.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/deracs/shares-rs
cd shares-rs
```

2. Create a `.env` file in the root directory:

```env
EMAIL=your_email@example.com
PASSWORD=your_password
RUST_LOG=off # info, debug, off
DB_TYPE=sqlite # surreal, sqlite, inmemory
```

### Running the Application

1. Build and run the application:

```bash
cargo run
```

### To Do

- Reauthenticate when token expires.
  - Currently when it 403's it just authenticates again.

### Contributing

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature/your-feature`.
5. Open a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Feel free to customize the README further according to your project's needs.
