# Warehouse Management System (Rust Backend)
## Description
This project is a Warehouse Management System built with Rust, Axum (a web framework for Rust), and SeaORM (an async ORM for Rust). It provides an API to manage warehouses and their items, with features like authentication using JWT tokens and role-based access control.

## Key Features
* Warehouse Management: Create, update, and delete warehouses.

* Item Management: Add, update, and remove items within warehouses.

* Authentication: Secure API endpoints using JWT (JSON Web Tokens).

* Database Integration: Uses SeaORM to interact with the database.

## Technologies Used
* Rust: A systems programming language focused on safety and performance.

* Axum: A web framework for building scalable and modular APIs in Rust.

* SeaORM: An async ORM for Rust, providing a type-safe and intuitive way to interact with databases.

* JWT: JSON Web Tokens for secure authentication.

* MySQL: Database used for persistent storage.

## Project Setup
### Prerequisites
* Rust: Install Rust from rustup.rs.

* MySQL: Ensure MySQL is installed and running locally.

* sea-orm-cli: Install the SeaORM CLI tool for managing migrations:

```
cargo install sea-orm-cli
```

### Database Configuration
Create a MySQL database named warehouse_db:

```
mysql -u root -p
CREATE DATABASE warehouse_db;
```

### Applying Migrations
SeaORM uses migrations to manage database schema changes. To apply the migrations, run:

```
sea-orm-cli migrate up
```
This will create the necessary tables in the warehouse_db database.

### Running the Application
After cloning the repository, build and run the project:

```
cargo run
```
The API will start on port 8080.

### Pending Tasks
API Documentation: Document the API endpoints to provide an interactive and user-friendly way to explore and test the API.

## Purpose
This project showcases my knowledge of Rust and its ecosystem, particularly in building performant and secure backend systems. It demonstrates my ability to work with modern web frameworks like Axum, manage databases using SeaORM, and implement authentication.
