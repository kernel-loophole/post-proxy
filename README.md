# Rust PostgreSQL Proxy API

This project provides a secure API for authenticating users and selectively exposing a PostgreSQL database. The API implements rate limiting and IP whitelisting to ensure that only authorized users can access their database schemas. It is designed for environments where exposing the database publicly is not an option, allowing users to opt into direct access.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Features

- **User Authentication**: Validate user credentials against the PostgreSQL database.
- **Rate Limiting**: Prevent abuse by limiting the number of login attempts.
- **IP Whitelisting**: Allow users to specify which IP addresses can access their database.
- **Opt-in Database Exposure**: Users can choose to expose their PostgreSQL database safely.

## Prerequisites

Make sure you have the following installed:

- Rust (latest stable version)
- PostgreSQL
- Cargo (comes with Rust installation)

## Getting Started

1. Clone the repository:

    ```bash
    git clone https://github.com/kernel-loophole/post-proxy.git
    cd post-proxy
    ```

2. Set up your PostgreSQL database and create the necessary tables for user credentials.

3. Update your `.env` file or configuration section in the code to include your database credentials.

4. Build the project:

    ```bash
    cargo build
    ```

5. Run the application:

    ```bash
    cargo run
    ```

## API Endpoints

### Authenticate User

- **Endpoint**: `POST /auth`
- **Description**: Authenticates a user based on the provided credentials and IP address.
- **Request Body**:

```json
{
    "psql": "your_psql_username",
    "password": "your_password",
    "ip": "user_ip_address",
    "db_host": "database_host",
    "db_user": "database_username",
    "db_password": "database_password",
    "db_name": "database_name"
}
```
```curl
curl -X POST http://127.0.0.1:8080/auth -H "Content-Type: application/json" -d '{"username": "postgres", "password": "new_password", "ip": "localhost", "db_host": "127.0.0.1", "db_user": "postgres", "db_password": "new_password", "db_name": "my_database"}'
```
