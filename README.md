
Vehicle Data Scraper
================

A Rust-based web scraper designed to collect detailed vehicle data from online listings, such as make, model, year, price, and various performance metrics. The data is stored in a database, allowing for easy analysis of trends and insights in the vehicle market.

Project Goals
-------------
- Gather comprehensive vehicle information, including:
  - Make, model, and year
  - Price
  - Horsepower, torque, weight
  - Fuel type, transmission, body type
  - (Potentially more fields)
- Store data in a database for easy querying and analysis.
- Provide insights and statistics, such as average prices, popular models, and performance trends.

Features
--------
- Web scraping of vehicle listings to gather relevant vehicle specifications.
- Database integration to store and manage vehicle data.
- Basic analysis tools to derive statistics and insights.

Getting Started
---------------

### Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **SQLite**: The initial implementation uses SQLite as the database, so make sure SQLite is installed.

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/AsbestosSoup-dev/vehicle-data-scraper.git
   cd vehicle-data-scraper
   ```
2. Install dependencies:
   ```bash
   cargo build
   ```

### Usage
1. Run the scraper:
   ```bash
   cargo run
   ```
2. Once the data is collected, you can query the database to retrieve insights.

Planned Enhancements
--------------------
- Support for additional vehicle attributes.
- Expanded data analysis tools for detailed trend analysis.
- CLI options for custom queries and filtering.

License
-------
This project is licensed under the MIT License—see the [LICENSE](LICENSE.txt) file for details.
