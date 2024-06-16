# rust-lambda-test

- `make build`: to build app
- `make run`: to build and test localy by curl requests
- `make invoke`: to invoke function (todo: explain better)



This project contains source code and supporting files for a serverless application that you can deploy with the SAM CLI. It includes the following files and folders:

- `rust_app/Cargo.toml` - Project configuration file.
- `rust_app/src/main.rs` - Code for the application's Lambda function.
- `template.yaml` - A template that defines the application's AWS resources.

The application uses several AWS resources, including Lambda functions and an API Gateway API. These resources are defined in the `template.yaml` file in this project. You can update the template to add AWS resources through the same deployment process that updates your application code.

