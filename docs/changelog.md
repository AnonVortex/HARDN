# Changelog

All notable changes to HARDN are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-06-16

### Added - Major Release: Headless-First Architecture
- **Headless Operation**: Complete whiptail removal for VM and server compatibility
- **Professional CLI**: Advanced command-line interface with full argument parsing
- **Native Debian Package**: Production-ready `.deb` package with dependency management
- **Modular Architecture**: 10 specialized security modules with clean separation
- **Systemd Integration**: Service management with comprehensive security restrictions
- **FHS Compliance**: Linux filesystem hierarchy standard compliance
- **Automated Backup/Restore**: Configuration management and rollback capabilities
- **CI/CD Pipeline**: Complete GitHub Actions workflow with multi-distribution testing
- **REST API**: HTTP API for remote monitoring and integration
- **Enhanced Logging**: Centralized logging with rotation and structured output

### Changed - Complete System Transformation
- **Architecture**: Transformed from monolithic 2,300+ line script to modular design
- **User Interface**: Professional CLI replacing interactive dialog dependencies
- **Installation**: Native package installation replacing manual script deployment
- **Configuration**: Centralized configuration management with templates
- **Security Model**: Enhanced STIG compliance with government-grade hardening
- **Documentation**: Comprehensive library with organized technical resources

### Security - Enterprise-Grade Hardening
- **STIG Compliance**: Complete DOD Security Technical Implementation Guide standards
- **Kernel Hardening**: 50+ kernel parameters for system security
- **Network Security**: Advanced firewall and intrusion prevention
- **Malware Protection**: Integrated detection and response capabilities
- **Audit Framework**: Comprehensive system auditing and compliance monitoring
- **Access Control**: Enhanced privilege separation and permission management

### Removed - Legacy Dependencies
- **Whiptail Dependency**: Eliminated for true headless operation
- **Interactive Prompts**: Replaced with intelligent defaults and automation
- **Manual Installation**: Superseded by professional package management
- Improved security hardening measures
- Added principle of least privilege throughout
- Strengthened system integrity monitoring

## [1.1.8] - Previous Release

### Added
- Enhanced system monitoring capabilities
- Improved performance optimization

### Fixed
- Resolved minor bugs from version 1.1.6

## [1.1.6] - Previous Release

### Added
- Internet connectivity verification
- Linux Malware Detect (maldet) integration
- Audit rules for critical system files

### Improved
- File permissions for critical system files
- System security configuration
- **Service Management**: Enhanced error handling and ensured `Fail2Ban`, `AppArmor`, and `auditd` are enabled and running at boot.
- **SSH Hardening**: Enforced stricter SSH settings for improved security.
- **Kernel Randomization**: Ensured kernel randomization is applied persistently and at runtime.

### Fixed
- **Error Handling**: Improved error handling for services like `Fail2Ban`, `AppArmor`, and `auditd` to prevent setup failures.


---

## Version 1.1.5

### Added
- **Debian Packaging**: Added support for building Debian packages for HARDN.
- **Error Handling**: Enhanced error handling in scripts to prevent disruptions to user logins or system functionality.

### Improved
- **Script Optimization**: Removed redundant steps and consolidated repetitive code blocks in setup scripts.
- **Documentation**: Updated documentation to reflect the latest changes and features.

### Fixed
- **Cron Jobs**: Ensured cron jobs are non-intrusive and do not disrupt user workflows.
- **GRUB BUG**: removed dependant file due to PAM collision and Kernal alerting flaw. 
- **AIDE Initialization**: Improved AIDE initialization process for better reliability.


---

*Note*: For detailed CLI usage instructions, refer to the [documentation](https://github.com/OpenSource-For-Freedom/HARDN/tree/main/docs).
