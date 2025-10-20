# NodePad-rs

## Description
NodePad-rs is a Rust desktop application for creating and visualizing notes as a graph, where each note can be linked to
others. One of its key features is the ability to set a customizable background, allowing you to anchor notes to
meaningful locations, such as placing notes about a specific place directly on a map image.

## Installation

### Local
* Download/clone the project from this repository
* Run the following command using Cargo
```console
cargo build --release
```
The executable will be located at ./target/release/NodePad-rs

### Web

#### Trunk
* Download/clone the project from this repository
* Run the following command using Trunk
```console
trunk build
```
The build products will be located in the ./dist/ directory

#### Docker
* Download/clone the project from this repository
* Run the following command using Docker
```console
docker build -t $IMAGE_NAME .
```
## Usage

### Local
This project can be run with Cargo by running the following command:
```console
cargo run
```
Alternately, if the project has been built using the Installation instructions above, the executable created can be run
as is appropriate for your OS, such as running ./NodePad-rs in Linux.

### Web

#### Trunk
This project can be run with Trunk by running the following command:
```console
trunk serve
```
This will create a test server accessible at http://localhost:8080/

Alternately, if the project has been built using the Installation instructions above, the build products can be run through your preferred web server, such as Nginx.

#### Docker
This project can be run with Docker after following the Docker installation instructions above with the following command:
```console
docker run -p 8080:80 $IMAGE_NAME
```
This will create a test server accessible at http://localhost:8080/

## Support
If there are any problems with this project, please email walden.hart.2003@gmail.com.

## Roadmap
* Create a UI
  * Create a basic UI showing nodes and their associated notes
  * Add a customizable background that can be used to add a map
  * Add interface to add new nodes

## Authors and acknowledgment
Walden Hart - Creator and Maintainer

## License
This project is licensed under the MIT License.
See the LICENSE file for full details.
