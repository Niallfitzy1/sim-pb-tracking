# Assetto Corsa Competizione Personal Best Tracker

This Rust application tracks personal best times set in Assetto Corsa Competizione, interacts with a PostgreSQL database
using SQLx, and sends messages to a Discord webhook.

## Prerequisites

- Rust and Cargo installed
- PostgreSQL database
- Discord webhook URL

## Setup

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Create a `.env` file in the root directory with the following content:
    ```dotenv
    DATABASE_URL="your_database_url"
    DRIVER_NAME="your_driver_name"
    DISCORD_WEBHOOK="your_discord_webhook_url"
    ```

3. Install dependencies:
    ```sh
    cargo build
    ```

## Running the Application

To run the application, use the following command:

```sh
cargo run
```