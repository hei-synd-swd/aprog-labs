# Rust Mystery Programs

This project is a collection of "mystery" programs written in Rust, each designed to demonstrate or stress-test different aspects of system performance, such as CPU, disk I/O, and GPU computation. The user can select which mystery program to run, and optionally enable verbose output to reveal the underlying implementation.

## Features

- **Mystery 1: Wait**
  Pauses execution for 2 seconds using a thread sleep.

- **Mystery 2: Fibonacci**
  Calculates the 45th Fibonacci number recursively, creating a CPU-intensive workload.

- **Mystery 3: Disk I/O**
  Performs heavy disk I/O by creating and then deleting 1000 temporary files.

- **Mystery 4: Parallel CPU**
  Utilizes all available CPU cores to perform a computationally intensive task in parallel.

- **Mystery 5: GPU (OpenCL)**
  Executes a simple SAXPY operation on the GPU using OpenCL, demonstrating basic GPU computation.

## Usage

Run the program and select a mystery (1-5) when prompted, or use command-line arguments:

```sh
cargo run -- --mystery 2 --verbose
```
