# Meeting Room Availability App

This Rust project shows the availability of meeting rooms for booking in a coworking center. The app configuration is done via environment variables that the user needs to define.

## Installation

1. Install Rust on your system. Visit https://www.rust-lang.org/tools/install for installation instructions.
2. Clone this repository: `git clone https://github.com/noodlensk/booking-rooms.git`.
3. Navigate to the project directory: `cd booking-rooms`.
4. Build the project: `cargo build`.
5. Run the project: `cargo run`.

## Configuration

The app configuration is done via the following environment variables that the user needs to define:

- `BOOKING_URL`: The URL of the booking system API.
- `BOOKING_USER`: The username for accessing the booking system.
- `BOOKING_PASSWORD`: The password for accessing the booking system.
- `BOOKING_ROOMS`: A comma-separated list of meeting rooms to check availability for.

## Usage

The app fetches the availability of meeting rooms from the booking system API and displays it in the terminal. It shows the status of each room as either available, booked or unavailable.

To run the app, enter `cargo run` on the terminal.
