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
cd rust-shares-api-sdk
```

2. Create a `.env` file in the root directory:

```env
EMAIL=your_email@example.com
PASSWORD=your_password
```

### Running the Application

1. Build and run the application:

```bash
cargo run
```

### Usage

#### Authenticate and Fetch Portfolio

```rust
use crate::presentation::sdk::SDK;
use dotenv::dotenv;
use std::env;
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let email = env::var("EMAIL").expect("EMAIL must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");

    let sdk = SDK::new();

    match sdk
        .authenticate(email.clone(), password.clone())
        .await
    {
        Ok(token) => {
            info!("Authentication successful: {:?}", token);
        }
        Err(err) => {
            info!("Authentication failed: {}", err);
        }
    }

    match sdk.get_portfolio().await {
        Ok(_) => info!("Portfolio retrieved successfully"),
        Err(err) => info!("Failed to retrieve portfolio: {}", err),
    }
}
```

### To Do

- Reauthenticate when token expires.

### Contributing

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature/your-feature`.
5. Open a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Feel free to customize the README further according to your project's needs.
