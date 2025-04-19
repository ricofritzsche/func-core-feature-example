
# Functional Core & Imperative Shell Example

This repository demonstrates **functional core and imperative shell** principles in a simple **asset tracking** and **geofencing** API. It uses **Rust** with a **vertical slice** architecture, but the approach could also apply to any modern language (like C# or Java).

Here is a full article: [Applying Functional Core and Imperative Shell in Practice](https://ricofritzsche.me/applying-functional-core-and-imperative-shell-in-practice/)

## Overview

- **Functional Core**: All domain logic lives in pure functions; no external dependencies, no side effects.  
- **Imperative Shell**: Deals with HTTP endpoints, database I/O, and other real-world concerns, calling into the functional core for actual business rules.  
- **Vertical Slice**: Each feature (e.g., `track_asset`) is self-contained. It contains its own domain model, logic, and handler code, making the project easy to extend.

## Use Case

We simulate an **asset** (like a truck or device) that reports its GPS coordinates. The system checks if the asset is **inside** or **outside** a predefined geofence:
1. The API endpoint receives the asset’s location.
2. The domain logic determines whether it’s inside or outside the boundary.
3. The system compares the new status to the old status and updates the database accordingly.

## Decisions

- **Rust** was chosen for its strong compiler guarantees, which encourage clear boundaries and safe concurrency.
- **Actix Web** provides the HTTP layer (the shell), while **SQLx** handles database interactions.
- **Vertical slices** keep each feature cohesive and avoid scattering domain logic across layers.
- **Functional Core** ensures the business logic is easy to test without spinning up databases or servers.

## Getting Started

1. **Install Rust**: [https://rustup.rs](https://rustup.rs)  
2. **Clone the repo**:
   ```bash
   git clone https://github.com/ricofritzsche/func-core-feature-example.git
   cd func-core-feature-example
   ```
3. **Set up your database** (e.g., PostgreSQL) and provide the URL via `.env` or an environment variable:
   ```env
   DATABASE_URL=postgres://user:password@localhost:5432/asset_tracking
   ```
4. **Run migrations** automatically on app startup:
   ```bash
   cargo run
   ```
   The code uses `sqlx::migrate!()` to apply any pending migrations under `./migrations`.

## Testing

- **Domain tests** (pure logic) run with:
  ```bash
  cargo test
  ```
- **Integration tests** use `reqwest` to hit the `/track` endpoint:
  ```bash
  cargo test --test track_asset_api
  ```
  Make sure the server is running (`cargo run`) if the test doesn’t spawn it automatically.

## Usage

Send a location update to the running server:

```bash
curl -X POST http://localhost:8080/track \
  -H "Content-Type: application/json" \
  -d '{"asset_id": "demo-asset", "lat": 40.5, "lon": -73.9}'
```

You should see a JSON response indicating whether the asset `Entered`, `Exited`, `StayedInside`, or `StayedOutside` the geofence.

## Contributing

Feel free to open issues or submit pull requests, especially if you have ideas to extend the feature set or demonstrate additional use cases.

## License

This project is licensed under the [MIT License](/Licence.md).

Enjoy experimenting with a functional core and imperative shell in your own domain!
