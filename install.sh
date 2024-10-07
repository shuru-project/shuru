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


# Function to show a simple progress indicator
show_progress() {
    while true; do
        printf "."
        sleep 1
    done
}

# Check if required dependencies are installed
check_dependencies() {
    command -v curl >/dev/null 2>&1 || { printf "${RED}curl is required but it's not installed. Please install curl and try again.${NC}\n"; exit 1; }
    command -v unzip >/dev/null 2>&1 || { printf "${RED}unzip is required but it's not installed. Please install unzip and try again.${NC}\n"; exit 1; }
    command -v tar >/dev/null 2>&1 || { printf "${RED}tar is required but it's not installed. Please install tar and try again.${NC}\n"; exit 1; }
}

# Function to install Shuru
install_shuru() {
    check_dependencies
    printf "${YELLOW}Checking for the latest release version...${NC}\n"

    # Fetch the latest release details from GitHub API
    LATEST_RELEASE_JSON=$(curl -s https://api.github.com/repos/shuru-project/shuru/releases/latest)

    # Extract the latest version tag from the JSON, handle API failures
    LATEST_VERSION=$(echo "$LATEST_RELEASE_JSON" | grep -oP '"tag_name": *"\K[^"]+')

    # Check if LATEST_VERSION is non-empty
    if [ -z "$LATEST_VERSION" ]; then
        printf "${RED}Failed to retrieve the latest version. Please check your internet connection or try again later.${NC}\n"
        exit 1
    fi

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

else
    echo "${RED}Unsupported operating system: $OS${NC}"
    exit 1
fi

# Get download URL
DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/shuru-project/shuru/releases/latest" | grep -o "\"browser_download_url\": *\"[^\"]*${FILE_EXTENSION}\"" | cut -d '"' -f 4)

# Print the download URL
echo "⬇️ ${YELLOW}Downloading shuru version $LATEST_VERSION for $OS...${NC}"

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
    echo "${RED}Unsupported file format for extraction${NC}"
    exit 1
fi

# Make the binary executable
chmod +x "$BINARY_PATH"

# Move the binary to a directory in the user's PATH
echo "🚀 ${YELLOW}Installing shuru into /usr/local/bin...${NC}"
sudo mv "$BINARY_PATH" /usr/local/bin/shuru

# Check if shuru binary exists in PATH
if command -v shuru >/dev/null 2>&1; then
    # Display installation complete message
    echo ""
    echo "✅ ${GREEN}shuru ${LATEST_VERSION} has been successfully installed.${NC}"
else
    echo "${RED}❌ Error: Failed to install shuru.${NC}"
    exit 1
fi

    # Extract the download URL
    DOWNLOAD_URL=$(echo "$LATEST_RELEASE_JSON" | grep -o "\"browser_download_url\": *\"[^\"]*${FILE_EXTENSION}\"" | cut -d '"' -f 4)

    # Check if the download URL is valid
    if [ -z "$DOWNLOAD_URL" ]; then
        printf "${RED}Failed to get the download URL for $FILE_EXTENSION. Please try again later.${NC}\n"
        exit 1
    fi

    # Print the download URL
    printf "⬇️ ${YELLOW}Downloading shuru version $LATEST_VERSION for $OS...${NC}"

    # Start showing progress
    show_progress &
    PROGRESS_PID=$!

    # Download the binary
    curl -LO "$DOWNLOAD_URL"

    # Stop the progress indicator
    kill $PROGRESS_PID >/dev/null 2>&1
    printf "\n"

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
