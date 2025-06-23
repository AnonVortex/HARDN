# HARDN v2.0.0: Security Hardening for Debian Systems

## Overview

HARDN v2.0.0 is a comprehensive security hardening solution designed for Debian-based systems. This release provides security compliance, threat detection, and protection features with full headless operation support for VM and server deployments.

## Key Features

### Headless Operation
- **Full VM Compatibility**: Designed for headless server and virtual machine deployments
- **Non-Interactive Mode**: Complete automation without user prompts
- **Default Security Settings**: Intelligent defaults for DNS (Quad9), firewall rules, and system hardening
- **Automated Installation**: Script-friendly installation and configuration

### Security Hardening
- **STIG Compliance**: Alignment with Security Technical Information Guides
- **Kernel Hardening**: 50+ kernel parameter optimizations
- **Network Security**: Comprehensive firewall and network protection
- **Access Control**: Enhanced user authentication and authorization
- **System Integrity**: File system and service protection

### Modular Architecture
- **Professional CLI**: Complete command-line interface with comprehensive options
- **Specialized Modules**: 9 security modules for different aspects of system hardening
- **REST API**: HTTP API for remote monitoring and integration
- **FHS Compliance**: Proper Linux filesystem hierarchy compliance

## Installed Packages and Tools

### Core Security Packages
- **ufw** - Uncomplicated Firewall for network protection
- **fail2ban** - Intrusion prevention system
- **apparmor**, **apparmor-profiles**, **apparmor-utils** - Mandatory access control
- **firejail** - Application sandboxing
- **tcpd** - TCP wrapper for service access control
- **lynis** - Security auditing tool
- **debsums** - Package integrity verification
- **libpam-pwquality** - Password quality enforcement

### Malware Detection and Response
- **rkhunter** - Rootkit detection and removal
- **chkrootkit** - Additional rootkit scanner
- **linux-malware-detect** (maldet) - Linux malware detection
- **aide**, **aide-common** - Advanced Intrusion Detection Environment
- **YARA** - Malware identification and classification rules

### System Tools
- **openssh-server**, **openssh-client** - Secure shell access
- **wget**, **curl**, **git**, **gawk** - Essential utilities
- **fwupd** - Firmware update daemon
- **policycoreutils** - SELinux policy utilities

### Virtualization Support
- **libvirt-daemon-system**, **libvirt-clients** - Virtualization management
- **qemu-system-x86** - Hardware emulation

## Security Features Implemented

### System Hardening & STIG Settings
- **Password Policy**: Minimum 14 characters, complexity requirements, retry limits
- **Account Security**: 35-day inactive account lockout
- **Login Banners**: Security warnings on system access
- **File Permissions**: Secured permissions on critical system files
- **Audit Framework**: Comprehensive system activity monitoring

### Kernel Security Parameters
- **ASLR** (Address Space Layout Randomization)
- **Exec Shield** protection
- **Kernel pointer restriction** (kptr_restrict)
- **DMesg restriction** for non-privileged users
- **Hardlink/Symlink protection**
- **ICMP and TCP security** enhancements
- **Source routing** disabled
- **IP forwarding** controls

### Network Security Configuration
- **Outbound Firewall Rules**: Restrictive rules allowing only essential traffic (DNS, NTP, updates)
- **IPv6 Disabled**: Reduces attack surface
- **UFW Configuration**: Strict inbound/outbound rules
- **Fail2Ban Jails**: SSH protection with custom ban settings

### Hardware Security
- **USB Storage Disabled**: Prevention of unauthorized data transfer
- **Core Dumps Disabled**: Protection against memory dump attacks
- **Ctrl+Alt+Del Disabled**: Prevention of unauthorized reboots
- **Firmware Updates**: Automated security updates via fwupd

## Malware Detection and Response

HARDN integrates multiple malware detection systems:

### Signature-Based Detection
- **Linux Malware Detect (LMD)**: Signature database for known malware
- **YARA Rules**: Pattern matching for malware identification
- **rkhunter**: Rootkit-specific detection signatures

### Behavioral Detection
- **AIDE**: File integrity monitoring for unauthorized changes
- **auditd**: System call monitoring and logging
- **AppArmor**: Application behavior restriction

### Response Capabilities
- **Automated Quarantine**: Suspicious files moved to secure location
- **Alert Generation**: Real-time notifications of security events
- **Log Aggregation**: Centralized security event logging
- **Recovery Tools**: System restoration capabilities

## Installation Structure

HARDN follows the Linux Filesystem Hierarchy Standard (FHS):

```
/usr/bin/hardn                    # Main executable
/usr/bin/hardn-api               # REST API server
/usr/share/hardn/modules/        # Security modules
├── logging.sh                   # Centralized logging
├── utils.sh                     # Common utilities
├── hardening.sh                 # Core security hardening
├── audit.sh                     # Security scanning
├── status.sh                    # System monitoring
├── backup.sh                    # Configuration backup
├── monitor.sh                   # Service management
├── update.sh                    # Security updates
└── uninstall.sh                 # Clean removal
/usr/share/hardn/templates/      # Configuration templates
/usr/share/man/man1/hardn.1      # Manual page
/etc/hardn/hardn.conf            # System configuration
/var/log/hardn/                  # Application logs
/var/lib/hardn/                  # Application data
/lib/systemd/system/             # Service files
```

## Command Line Interface

### Primary Commands
```bash
# System hardening
sudo hardn setup [--non-interactive]     # Install and configure security
sudo hardn setup --dry-run              # Preview changes

# System monitoring
hardn status                             # Current security status
hardn audit                              # Security compliance scan
hardn monitor [start|stop|status]       # Service monitoring

# Configuration management
hardn backup [create|restore|list]      # Configuration backup
hardn config [show|edit|validate]       # Configuration management
hardn update                             # Security signature updates

# Service management  
hardn api [--port PORT]                 # Start REST API server
hardn uninstall [--purge]               # Clean system removal
```

### Advanced Options
- `--non-interactive`: Headless operation without prompts
- `--dry-run`: Preview changes without execution
- `--force`: Skip confirmation prompts
- `--verbose`: Detailed output
- `--config FILE`: Custom configuration file

## Monitoring & Reporting

### Log Files
- `/var/log/hardn/hardn.log` - Main application log
- `/var/log/hardn/audit.log` - Security audit events
- `/var/log/hardn/monitor.log` - Service monitoring
- `/var/log/hardn/api.log` - REST API access log

### Automated Tasks
- **Daily Security Scans**: Automated malware and compliance checks
- **Weekly Signature Updates**: Security definition updates
- **Monthly System Audits**: Comprehensive security assessments
- **Log Rotation**: Automated log file management

### Alert System
- **Real-time Monitoring**: Continuous security event detection
- **Configurable Thresholds**: Customizable alert triggers  
- **Multiple Channels**: Log files, email, API notifications
- **Escalation Procedures**: Graduated response to security events

## REST API Integration

The HARDN API provides remote monitoring and management capabilities:

### Endpoints
- `GET /api/status` - System security status
- `GET /api/services` - Security services status
- `GET /api/metrics` - Performance and security metrics
- `GET /api/logs` - Security event logs
- `POST /api/service` - Service control operations

### Usage
```bash
# Start API server
hardn api --port 8080 --host 0.0.0.0

# Query system status
curl http://localhost:8080/api/status

# Check services
curl http://localhost:8080/api/services
```

## Compliance and Standards

### Security Standards Implemented
- **NIST Cybersecurity Framework**: Core security functions
- **CIS Controls**: Center for Internet Security benchmarks
- **STIG Guidelines**: Defense Information Systems Agency standards
- **ISO 27001**: Information security management principles

### Compliance Metrics
- **Lynis Security Score**: Target 99%+ (up from baseline ~57%)
- **STIG Compliance**: 95%+ of applicable controls
- **CIS Benchmarks**: Level 2 implementation
- **Vulnerability Assessment**: Regular scanning and remediation

## Technical Specifications

### System Requirements
- **Operating System**: Debian 12, Ubuntu 24.04/22.04
- **Architecture**: x86_64 (amd64)
- **Memory**: 1GB minimum, 2GB recommended
- **Storage**: 500MB free space
- **Network**: Internet connectivity for updates

### Performance Impact
- **CPU Usage**: <5% during normal operation
- **Memory Footprint**: ~50MB resident memory
- **Storage Overhead**: ~200MB for logs and data
- **Network Bandwidth**: Minimal for signature updates

### Compatibility
- **Virtualization**: VMware, VirtualBox, KVM, Hyper-V
- **Cloud Platforms**: AWS, Azure, GCP, DigitalOcean
- **Container Support**: Docker, LXC, systemd-nspawn
- **Automation Tools**: Ansible, Puppet, Chef compatible

## Support and Maintenance

### Update Mechanism
- **Automatic Updates**: Security signature updates
- **Package Updates**: Debian package management integration
- **Configuration Sync**: Centralized configuration management
- **Rollback Capability**: Configuration restoration

### Documentation
- **Manual Page**: `man hardn` - Complete command reference
- **Online Documentation**: Comprehensive guides and tutorials
- **API Documentation**: REST API reference
- **Community Support**: GitHub issues and discussions

## Version History

### v2.0.0 (Current)
- Complete headless operation support
- Modular architecture with 9 specialized security modules
- Professional CLI interface with comprehensive options
- REST API for remote monitoring and management
- Enhanced STIG compliance and security hardening
- Native Debian package with proper FHS compliance

### Previous Versions
- v1.x: Original monolithic script implementation
- Legacy versions deprecated in favor of v2.0.0 architecture

## License and Support

- **License**: MIT License - Open source and freely distributable
- **Support**: Community support via GitHub issues
- **Commercial Support**: Available through CyberSynapse
- **Contact**: office@cybersynapse.ro

---

**HARDN v2.0.0** - Professional Security Hardening for Debian Systems  
*Headless • Compliant • Modular • Enterprise-Ready*
