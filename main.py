import subprocess
from collections import defaultdict
import sys


def count_lines_starting_with_l(data):
    path, line = data.split("|", 1)  # Split the received data into path and line
    return path, 1 if line.startswith("l") else 0


def run_rust_script(folder_path, interval_secs=None):
    # Build the command to call the Rust executable
    command = ["./target/release/monitor_folder", folder_path]

    # If interval is provided, append it to the command
    if interval_secs:
        command.append(str(interval_secs))

    # Call the Rust script with the folder path and optional interval as arguments
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    file_counts = defaultdict(int)  # Dictionary to store counts for each file

    try:
        # Continuously read output
        while True:
            output = process.stdout.readline()
            if output:
                output = output.strip()
                # Count lines starting with "l" and get the file path
                path, count = count_lines_starting_with_l(output)
                file_counts[path] += count

                # Print the current count for the specific file
                print(f"File: {path}, Lines starting with 'l': {file_counts[path]}")
            else:
                break

    except KeyboardInterrupt:
        # If you want to handle stopping the process manually (e.g., via Ctrl+C)
        print("Process interrupted. Killing the Rust process.")
        process.kill()

    # Check if any errors occurred
    stderr = process.stderr.read()
    if stderr:
        print(f"Rust script error: {stderr}", file=sys.stderr)


if __name__ == "__main__":
    folder_to_monitor = "demo"

    # Optional: set interval in seconds, or leave as None for running once
    interval = 10  # Set to None to run once, or specify the interval in seconds

    run_rust_script(folder_to_monitor, interval)
