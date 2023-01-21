# Actix-web "Hello World" REST API

This is a simple "hello world" REST API built with the actix-web framework for Rust. It listens for GET requests on the root path ("/") and responds with "Hello, World!".

## Building and Running the App

To build and run this app, you will need to have Docker and Rust installed on your machine.

1. Clone this repository to your local machine:
   1. `git clone https://github.com/jmelm93/rust-actixweb-api`
2. Navigate to the project directory and build the Docker image using the following command:
   1. `docker build -t rust-actixweb-api .`
3. Start a container from the image and map port 8000 from the container to port 8000 on your host machine:
   1. `docker run -p 8000:8000 rust-actixweb-api`
4. Test the API by sending a GET request to `http://localhost:8000/` using a tool like `curl` or by opening the URL in a web browser. You should see the response "Hello, World!"

## Deploying the App

To deploy this app, you will need to have a server with Docker installed (or docker on your local machine). Once you have a server ready, you can follow the same steps as building and running the app on your local machine.

