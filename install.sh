#!/bin/sh

# Set text colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Function to handle cleanup on interrupt (Ctrl+C)
cleanup() {
    echo ""
    echo "${RED}Installation interrupted.${NC}"
    exit 1
}

# Trap SIGINT (Ctrl+C) and call the cleanup function
trap cleanup INT

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
        echo "${RED}Unsupported architecture: $ARCH${NC}"
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
    tar -xzf "$(basename "$DOWNLOAD_URL")"
    EXTRACTED_DIR="./${LATEST_VERSION}_${ARCH}-unknown-linux-musl"
    BINARY_PATH="${EXTRACTED_DIR}/shuru"  # Adjusted binary path
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

# Clean up downloaded zip file
rm -f "$ZIP_FILE"
