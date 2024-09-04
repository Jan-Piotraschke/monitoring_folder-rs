# Monitoring a folder for changes

This is a simple program that monitors a folder for changes of its .txt and .log files. 

## Compile

```bash
cargo build --release
```

or for a Windows target

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Run on MacOS

If you want to test your windows cross-compiled binary on MacOS, you can use wine:

```bash
wine ./target/x86_64-pc-windows-gnu/release/monitor_folder.exe demo 10
```

Naturally, you can also analyse the logs from your target folder in python similiar to the following:

```bash
python main.py
```

Feel free to adapt the python logic to your needs.
