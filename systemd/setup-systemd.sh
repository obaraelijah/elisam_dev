#!/bin/bash

# Create the systemd user directory if it doesn't exist
mkdir -p ~/.config/systemd/user

# Copy the service files to the systemd user directory
cp *.service ~/.config/systemd/user/

# Make sure systemd doesn't kill the services after you log out
loginctl enable-linger $(whoami)

# Reload systemd to read the new service files
systemctl --user daemon-reload

# Enable and start the services
systemctl --user enable run_site
systemctl --user enable run_caddy
systemctl --user start run_site
systemctl --user start run_caddy

# Provide instructions for checking logs
echo "Check logs with:"
echo "journalctl --user -u run_caddy"
echo "journalctl --user -u run_site"
