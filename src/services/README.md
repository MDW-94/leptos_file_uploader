# Services Notes

## Setting Up Services

Our application needs to interact with a third party API to handle file uploads.

In this section we will set up the dependencies and organize our services.

-serde: Used to serialize and deserialize data structures, enabling us to prepare our data for API requests and handle the responses.
-reqwasm: A WebAssembly compatible HTTP client that allows us to send HTTP requests directly from the browser.
-web-sys: Provides bindings to the browser’s native File API for handling file objects.

Add these dependencies with the following cmd:

```bash
cargo add web-sys --features=File
cargo add serde --features=derive
cargo add reqwasm
```

## [Defining Data Types](./types.rs)

To facilitate communication with the API, we will define data types that represent the expected request and response structures. This approach allows us to serialize and deserialize data accurately when interacting with the API.

- FailureReply: Represents error messages returned from the API when a request fails.

- BucketDetail: Represents the response structure for successful file transfers, including the bucket_id field, which uniquely identifies the file location.

## [Implementing the transfer service](./transferer.rs)

Now, let’s create the main function for file transfer in transferer.rs. This function will prepare the files selected by the user, package them into a FormData, and send them as a multipart/form-data request to the API.

The transfer_file function performs the following steps:

1. Initialize FormData: The FormData object allows us to format files as multipart/form-data for API transmission.
2. Add files to FormData: Each selected file by the user is appended to the FormData instance.
3. Send the HTTP request: Using reqwasm, we send a POST request to the API_URL, including the FormData as the request body.
4. Handle the response: We expect a successful response to contain a BucketDetail with a bucket_id, or a failure response to return a FailureReply with an error message.

## Random Rust Fixes

### Rust Analyzer

```bash
rustup +nightly component add rust-analyzer
```

cmd + p

restart Rust Analyzer server

create [rust-toolchain.toml](../../rust-toolchain.toml)
