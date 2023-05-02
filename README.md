# gpio-daemon
A simple daemon to listen for inputs on a GPIO pin and run a command.

Prerequesites
Rust toolchain
GPIO device using the linux userspace GPIO interface, such as on the Raspberry Pi

# Installation

## Clone the repository:
```
git clone https://github.com/erickloeckner/gpio-daemon
```

## Move to the newly created directory and build the binary:
```
cd gpio-daemon
cargo build --release
```

## Edit the systemd service file gpio-daemon.service found in the root directory of the repository:

### Change the ExecStart line to run the compiled binary. The first argument is the path to the configuration file. In the example I cloned the repo to /home/pi/rust/ on a Pi Zero running Raspberry Pi OS 10:
Original:
```
ExecStart=$path_to_binary $path_to_config.toml
```

Example:
```
ExecStart=/home/pi/rust/gpio-daemon/target/release/gpio-daemon /home/pi/rust/gpio-daemon/config.toml
```

### Change the User line to the owner of the executable. The user must be in the gpio group or otherwise be able to access the GPIO interface:
Original:
```
User=$user
```

Example:
```
User=pi
```

## Copy the service file to one of the locations checked by systemd:
```
sudo cp gpio-daemon.service /etc/systemd/system/gpio-daemon.service
```

## Start and enable the service:
```
sudo systemctl start gpio-daemon.service
sudo systemctl enable gpio-daemon.service
```
