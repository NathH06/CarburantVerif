# CarburantVerif
A short rust application with french carburant data.

### Description
The application is a cli tool that allows you to get the lowest fuel prices for a specific fuel type and location in France. It also list all fuel stations with their data in a specific city or postal code or address. The data is fetched from the French government data.gouv.fr database and is updated regularly to ensure accuracy.

### Features
- Fetch fuel prices from the French government data.
- Retrieve information about gas stations, including location and available fuel types.
- Support for multiple fuel types, including:
    - SP95
    - SP98
    - E10
    - E85
    - Diesel
- get lowest fuel prices for a specific fuel type and location.

### Installation
To use this application, you need to have Rust installed on your system. You can install Rust by following the instructions on the official website: https://www.rust-lang.org/tools/install

and then clone this repository to your local machine. You can do this by running the following command in your terminal:
```bash
git clone ...
```
and then navigate to the project directory:
```bash
cd carburant_verif
```

and then build the project using Cargo:
```bash
cargo build --release
```

and after building the project, you can run the application using the following command:
```bash
cargo run --release
```

### Future features
- coming soon: allows you to get the highest fuel prices for a specific fuel type and location.
- coming soon: allows you to get the average fuel prices for a specific fuel type and location.

### Authors and acknowledgment
- Author of the library: Nathan H
- Author of the application code: Nathan H
- Author of the data: French government Ministères économiques et financiers (https://www.data.gouv.fr/api/1/datasets/r/b0561905-7b5e-4f38-be50-df05708acb80)
- All french stations for sharing their data with the public.
- All developers who created open source libraries and tools that helped in the development of this library.

### License
This project is licensed under the GNU Affero General Public License v3.0 - see the [LICENSE](LICENSE.md) file for details.

### warning
This application and the library it uses are created for educational purposes only. The author is not responsible for any misuse of the data or any consequences that may arise from using this application and library. Please use the data responsibly and in accordance with applicable laws and regulations.