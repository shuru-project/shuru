#!/usr/bin/env sh

set -eu

# Set text colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# ASCII art for the Shuru project
printf "\n"
printf "███████╗██╗  ██╗██╗1   ██╗██████╗ ██╗   ██╗    ██████╗ ██████╗  ██████╗      ██╗███████╗ ██████╗████████╗\n"
printf "██╔════╝██║  ██║██║   ██║██╔══██╗██║   ██║    ██╔══██╗██╔══██╗██╔═══██╗     ██║██╔════╝██╔════╝╚══██╔══╝\n"
printf "███████╗███████║██║   ██║██████╔╝██║   ██║    ██████╔╝██████╔╝██║   ██║     ██║█████╗  ██║        ██║   \n"
printf "╚════██║██╔══██║██║   ██║██╔══██╗██║   ██║    ██╔═══╝ ██╔══██╗██║   ██║██   ██║██╔══╝  ██║        ██║   \n"
printf "███████║██║  ██║╚██████╔╝██║  ██║╚██████╔╝    ██║     ██║  ██║╚██████╔╝╚█████╔╝███████╗╚██████╗   ██║   \n"
printf "╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝     ╚═╝     ╚═╝  ╚═╝ ╚═════╝  ╚════╝ ╚══════╝ ╚═════╝   ╚═╝   \n"
printf "${RED}\nShuru Project: A task runner and Node.js/Python version manager written in Rust${NC}\n\n"

# Function to handle cleanup on interrupt (Ctrl+C)
cleanup() {
    printf "\n${RED}Installation interrupted.${NC}\n"
    exit 1
}

# Trap SIGINT (Ctrl+C) and call the cleanup function
trap cleanup INT

# Function to install Shuru
install_shuru() {
    printf "${YELLOW}Checking for the latest release version...${NC}\n"
    
    # Set the latest release version
    LATEST_VERSION=$(curl -s "https://api.github.com/repos/shuru-project/shuru/releases/latest" | grep -o '"tag_name": "v.*"' | cut -d'"' -f4)

    # Determine the operating system and architecture
    OS=$(uname -s)
    ARCH=$(uname -m)

    # Set the file extension based on the operating system
    if [ "$OS" = "Darwin" ]; then
        FILE_EXTENSION="apple-darwin.zip"
    elif [ "$OS" = "Linux" ]; then
        if [ "$ARCH" = "x86_64" ]; then
            FILE_EXTENSION="unknown-linux-musl.tar.gz"
        else
            printf "${RED}Unsupported architecture: $ARCH${NC}\n"
            exit 1
        fi
    else
        printf "${RED}Unsupported operating system: $OS${NC}\n"
        exit 1
    fi

    # Get download URL
    DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/shuru-project/shuru/releases/latest" | grep -o "\"browser_download_url\": *\"[^\"]*${FILE_EXTENSION}\"" | cut -d '"' -f 4)

    # Print the download URL
    printf "⬇️ ${YELLOW}Downloading shuru version $LATEST_VERSION for $OS...${NC}\n"

    # Download the binary
    curl -LO "$DOWNLOAD_URL"

    # Extract the binary if it's a tarball or zip
    if echo "$DOWNLOAD_URL" | grep -q ".tar.gz"; then
        ZIP_FILE=$(basename "$DOWNLOAD_URL")
        tar -xzf "$ZIP_FILE"
        BINARY_PATH="./shuru"
    elif echo "$DOWNLOAD_URL" | grep -q ".zip"; then
        ZIP_FILE=$(basename "$DOWNLOAD_URL")
        unzip "$ZIP_FILE"
        BINARY_PATH="./shuru"
    else
        printf "${RED}Unsupported file format for extraction${NC}\n"
        exit 1
    fi

    # Make the binary executable
    chmod +x "$BINARY_PATH"

    # Move the binary to a directory in the user's PATH
    printf "🚀 ${YELLOW}Installing shuru into /usr/local/bin...${NC}\n"

    sudo mv "$BINARY_PATH" /usr/local/bin/shuru

    # Check if shuru binary exists in PATH
    if command -v shuru >/dev/null 2>&1; then
        printf "\n✅ ${GREEN}shuru ${LATEST_VERSION} has been successfully installed.${NC}\n"
    else
        printf "${RED}❌ Error: Failed to install shuru.${NC}\n"
        exit 1
    fi

    # Clean up downloaded zip file
    rm -f "$ZIP_FILE"
}

# Function to uninstall Shuru
uninstall_shuru() {
    printf "${YELLOW}Uninstalling Shuru...${NC}\n"
    if [ -f /usr/local/bin/shuru ]; then
        sudo rm -f /usr/local/bin/shuru
        printf "${GREEN}Shuru has been successfully uninstalled.${NC}\n"
    else
        printf "${RED}Shuru is not installed.${NC}\n"
    fi
}

# Function to handle the reinstallation
reinstall_shuru() {
    uninstall_shuru
    install_shuru
}

# Interactive menu
printf "Please select an option:\n"
printf "1) Install\n"
printf "2) Re-install\n"
printf "3) Uninstall\n"
read -p "Enter your choice [1-3]: " choice

case $choice in
    1)
        install_shuru
        ;;
    2)
        reinstall_shuru
        ;;
    3)
        uninstall_shuru
        ;;
    *)
        printf "${RED}Invalid option. Exiting...${NC}\n"
        exit 1
        ;;
esac
