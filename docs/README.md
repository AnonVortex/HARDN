# HARDN Documentation

Welcome to the HARDN v2.0.0 documentation directory. This collection provides comprehensive information about the HARDN security hardening system with full headless operation support.

## 📚 Documentation Library

**All comprehensive documentation has been moved to the [Documentation Library](library/README.md)**

The library provides organized access to:
- **Installation Guides** - Complete setup instructions
- **Usage Documentation** - Headless and interactive operation
- **Security References** - Compliance and tool integration
- **Development Resources** - Architecture and contribution guides

## 🚀 Quick Start

**New to HARDN?** Visit the [Documentation Library](library/README.md) for organized access to all resources.

## Core Files

- **[HARDN.md](HARDN.md)** - Main system documentation and package details
- **[Table of Contents](TABLE_OF_CONTENTS.md)** - Complete documentation index  
- **[Changelog](changelog.md)** - Version history and updates

## Key Features of v2.0.0

### ✅ Headless-First Design
- **VM Compatible**: Perfect for virtual machine deployments
- **No User Interaction**: Automated security hardening
- **Server Ready**: Optimized for headless server environments

### ✅ Professional Package
- **Native Debian Package**: Proper dependency management
- **System Integration**: Full systemd and FHS compliance
- **Enterprise Architecture**: Modular, maintainable design

## Quick Installation

```bash
# Download and install latest package
wget https://github.com/OpenSource-For-Freedom/HARDN/releases/latest/download/hardn_2.0.0-1_all.deb
sudo dpkg -i hardn_2.0.0-1_all.deb

# Headless hardening (perfect for VMs)
sudo hardn setup --non-interactive
```

## Support

For detailed guidance, visit the [Documentation Library](library/README.md) or refer to the main [README](../README.md).
