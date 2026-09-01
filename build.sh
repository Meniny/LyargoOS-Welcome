#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}==> LyargoOS Welcome Build Script${NC}"
echo ""

# Dependencies to check
DEPS=(
    "cargo:rust"
    "blueprint-compiler:blueprint-compiler"
    "pkg-config:pkg-config"
    "gtk4-demo:gtk4-devel"
)

MISSING=()

echo "==> Checking dependencies..."
for dep in "${DEPS[@]}"; do
    bin="${dep%%:*}"
    pkg="${dep##*:}"
    if command -v "$bin" &>/dev/null; then
        echo -e "  ${GREEN}✓${NC} $bin"
    else
        echo -e "  ${RED}✗${NC} $bin (provided by: $pkg)"
        MISSING+=("$pkg")
    fi
done

# Check gtk4 via pkg-config
if pkg-config --exists gtk4 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} gtk4 ($(pkg-config --modversion gtk4))"
else
    echo -e "  ${RED}✗${NC} gtk4 not found via pkg-config"
    if [[ ! " ${MISSING[@]} " =~ "gtk4-devel" ]]; then
        MISSING+=("gtk4-devel")
    fi
fi

# Check glib2 via pkg-config
if pkg-config --exists glib-2.0 2>/dev/null; then
    echo -e "  ${GREEN}✓${NC} glib2 ($(pkg-config --modversion glib-2.0))"
else
    echo -e "  ${RED}✗${NC} glib2 not found via pkg-config"
    if [[ ! " ${MISSING[@]} " =~ "glib-devel" ]]; then
        MISSING+=("glib-devel")
    fi
fi

echo ""

# Install missing dependencies
if [ ${#MISSING[@]} -gt 0 ]; then
    echo -e "${YELLOW}==> Missing dependencies: ${MISSING[*]}${NC}"
    
    # Detect package manager
    if command -v xbps-install &>/dev/null; then
        echo -e "${GREEN}==> Installing via xbps...${NC}"
        sudo xbps-install -S ${MISSING[*]}
    elif command -v apt-get &>/dev/null; then
        echo -e "${GREEN}==> Installing via apt...${NC}"
        # Map Void packages to Debian packages
        APT_DEPS=()
        for pkg in "${MISSING[@]}"; do
            case "$pkg" in
                rust) APT_DEPS+=("cargo") ;;
                blueprint-compiler) APT_DEPS+=("blueprint-compiler") ;;
                pkg-config) APT_DEPS+=("pkg-config") ;;
                gtk4-devel) APT_DEPS+=("libgtk-4-dev") ;;
                glib-devel) APT_DEPS+=("libglib2.0-dev") ;;
                *) APT_DEPS+=("$pkg") ;;
            esac
        done
        sudo apt-get install -y ${APT_DEPS[*]}
    elif command -v dnf &>/dev/null; then
        echo -e "${GREEN}==> Installing via dnf...${NC}"
        sudo dnf install -y ${MISSING[*]}
    elif command -v pacman &>/dev/null; then
        echo -e "${GREEN}==> Installing via pacman...${NC}"
        # Map to Arch packages
        PAC_DEPS=()
        for pkg in "${MISSING[@]}"; do
            case "$pkg" in
                rust) PAC_DEPS+=("rust") ;;
                blueprint-compiler) PAC_DEPS+=("blueprint-compiler") ;;
                pkg-config) PAC_DEPS+=("pkgconf") ;;
                gtk4-devel) PAC_DEPS+=("gtk4") ;;
                glib-devel) PAC_DEPS+=("glib2") ;;
                *) PAC_DEPS+=("$pkg") ;;
            esac
        done
        sudo pacman -S --needed ${PAC_DEPS[*]}
    else
        echo -e "${RED}==> Unknown package manager. Please install manually:${NC}"
        echo "   ${MISSING[*]}"
        exit 1
    fi
    echo ""
fi

# Build
echo -e "${GREEN}==> Configuring...${NC}"
./configure --prefix=/usr

echo ""
echo -e "${GREEN}==> Building...${NC}"
make

echo ""
echo -e "${GREEN}==> Build complete!${NC}"
echo ""
echo "To install:"
echo "  sudo make install"
echo ""
echo "To run without installing:"
echo "  cargo run"
