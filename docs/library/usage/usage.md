# HARDN Usage Guide

This guide explains how to use HARDN v2.0.0 for system security hardening and monitoring, including headless operation for VM and server deployments.

## Quick Start

### Installation
```bash
# Download and install HARDN
wget https://github.com/OpenSource-For-Freedom/HARDN/releases/latest/download/hardn_2.0.0-1_all.deb
sudo dpkg -i hardn_2.0.0-1_all.deb
sudo apt-get install -f
```

### Basic Usage
```bash
# Interactive setup (recommended for first-time users)
sudo hardn setup

# Headless setup (for VM/server deployments)
sudo hardn setup --non-interactive

# Check system status
hardn status

# Run security audit
hardn audit
```

## System Hardening

### Interactive Mode
For first-time users or when you want to review changes:

```bash
# Full interactive setup
sudo hardn setup

# Interactive setup with preview
sudo hardn setup --dry-run
```

### Headless Mode
HARDN v2.0.0 supports full headless operation for automated deployments:

```bash
# Non-interactive setup (perfect for VMs and servers)
sudo hardn setup --non-interactive

# Force setup even if already configured
sudo hardn setup --non-interactive --force

# Headless setup with custom configuration
sudo hardn setup --non-interactive --config /path/to/config.conf
```

**Key v2.0.0 Headless Features:**
- **No User Interaction Required**: Automated security hardening without prompts
- **Intelligent Defaults**: Automatic DNS configuration (Quad9), firewall rules, and security settings
- **VM Optimized**: Designed specifically for virtual machine and server deployments
- **Container Compatible**: Works in Docker and other containerized environments

## System Monitoring

### Status Checking
```bash
# Basic status check
hardn status

# Detailed status with service information
hardn status --verbose

# Check specific components
hardn status --services      # Security services status
hardn status --firewall      # Firewall configuration
hardn status --compliance    # Compliance status
```

### Security Auditing
```bash
# Complete security audit
hardn audit

# Quick audit (faster, less comprehensive)
hardn audit --quick

# Generate detailed compliance report
hardn audit --report

# Audit specific components
hardn audit --lynis          # Lynis security scan only
hardn audit --stig           # STIG compliance check
```

### Service Management
```bash
# Start monitoring services
hardn monitor start

# Stop monitoring
hardn monitor stop

# Check monitoring status
hardn monitor status

# Restart specific services
hardn monitor restart fail2ban
hardn monitor restart firewall
hardn monitor restart apparmor
```

## Configuration Management

### Configuration Commands
```bash
# View current configuration
hardn config show

# Edit configuration interactively
hardn config edit

# Validate configuration
hardn config validate

# Reset to default configuration
hardn config reset

# Use custom configuration file
hardn --config /path/to/custom.conf setup
```

### Configuration Files
HARDN uses the following configuration files:

- `/etc/hardn/hardn.conf` - Main configuration file
- `/etc/hardn/services.conf` - Service-specific settings
- `/etc/hardn/monitoring.conf` - Monitoring configuration
- `/var/lib/hardn/state.json` - System state information

## Backup and Restore

### Backup Operations
```bash
# Create configuration backup
hardn backup create

# Create backup with custom name
hardn backup create --name "pre-update-$(date +%Y%m%d)"

# List available backups
hardn backup list

# Show backup details
hardn backup show backup-20250616-143022
```

### Restore Operations
```bash
# Restore from most recent backup
hardn backup restore

# Restore from specific backup
hardn backup restore backup-20250616-143022

# Preview restore without applying
hardn backup restore --dry-run backup-20250616-143022

# Clean old backups (keep last 10)
hardn backup clean --keep 10
```

## Update Management

### Security Updates
```bash
# Update security signatures and definitions
hardn update

# Update system packages
hardn update --packages

# Check for available updates
hardn update --check

# Update with automatic reboot if needed
hardn update --auto-reboot
```

### Signature Management
```bash
# Update malware signatures
hardn update --signatures

# Update YARA rules
hardn update --yara

# Update rkhunter database
hardn update --rkhunter

# Force signature update
hardn update --force
```

## REST API

### Starting the API Server
```bash
# Start API server (default: localhost:8080)
hardn api

# Start on custom port and host
hardn api --port 9090 --host 0.0.0.0

# Start with SSL/TLS
hardn api --ssl --cert /path/to/cert.pem --key /path/to/key.pem

# Start in background
hardn api --daemon
```

### API Endpoints
```bash
# System status
curl http://localhost:8080/api/status

# Security services status
curl http://localhost:8080/api/services

# Live system metrics
curl http://localhost:8080/api/metrics

# Security logs
curl http://localhost:8080/api/logs

# Control services
curl -X POST http://localhost:8080/api/service \
  -H "Content-Type: application/json" \
  -d '{"service": "fail2ban", "action": "restart"}'
```

## Command Line Options

### Global Options
- `--help` - Show help information
- `--version` - Display version information
- `--config FILE` - Use custom configuration file
- `--verbose` - Enable verbose output
- `--quiet` - Suppress non-essential output
- `--dry-run` - Show what would be done without making changes

### Setup Options
- `--non-interactive` - Run without user prompts (headless mode)
- `--force` - Force installation even if system is already hardened
- `--skip-packages` - Skip package installation
- `--skip-config` - Skip configuration changes
- `--container-mode` - Optimize for container deployment

### Monitoring Options
- `--interval SECONDS` - Set monitoring interval (default: 30)
- `--log-level LEVEL` - Set logging level (debug, info, warn, error)
- `--output FORMAT` - Output format (text, json, xml)

## Automation and Scripting

### Automated Deployment Script
```bash
#!/bin/bash
# HARDN automated deployment

set -e

# Download and install
wget -q https://github.com/OpenSource-For-Freedom/HARDN/releases/latest/download/hardn_2.0.0-1_all.deb
sudo dpkg -i hardn_2.0.0-1_all.deb 2>/dev/null || true
sudo apt-get install -f -y

# Configure in headless mode
sudo hardn setup --non-interactive --force

# Enable monitoring service
sudo systemctl enable --now hardn-monitor.service

# Verify deployment
if hardn status --quiet; then
    echo "✓ HARDN deployed successfully"
    hardn status --verbose
else
    echo "✗ HARDN deployment failed"
    exit 1
fi
```

### Ansible Integration
```yaml
---
- name: Deploy HARDN Security Hardening
  hosts: debian_servers
  become: yes
  tasks:
    - name: Download HARDN package
      get_url:
        url: https://github.com/OpenSource-For-Freedom/HARDN/releases/latest/download/hardn_2.0.0-1_all.deb
        dest: /tmp/hardn_2.0.0-1_all.deb

    - name: Install HARDN package
      apt:
        deb: /tmp/hardn_2.0.0-1_all.deb
        state: present

    - name: Configure HARDN
      command: hardn setup --non-interactive --force

    - name: Enable HARDN monitoring
      systemd:
        name: hardn-monitor.service
        enabled: yes
        state: started

    - name: Verify HARDN status
      command: hardn status --quiet
      register: hardn_status
      failed_when: hardn_status.rc != 0
```

## Docker Integration

### Dockerfile Example
```dockerfile
FROM debian:12-slim

# Install HARDN
RUN apt-get update && \
    apt-get install -y wget curl && \
    wget -q https://github.com/OpenSource-For-Freedom/HARDN/releases/latest/download/hardn_2.0.0-1_all.deb && \
    dpkg -i hardn_2.0.0-1_all.deb || true && \
    apt-get install -f -y && \
    rm hardn_2.0.0-1_all.deb

# Configure HARDN for container
RUN hardn setup --non-interactive --container-mode

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD hardn status --quiet || exit 1

# Start monitoring
CMD ["hardn", "monitor", "start"]
```

### Docker Compose
```yaml
version: '3.8'

services:
  hardn-server:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - hardn-logs:/var/log/hardn
      - hardn-data:/var/lib/hardn
    environment:
      - HARDN_LOG_LEVEL=info
      - HARDN_API_PORT=8080
    restart: unless-stopped

volumes:
  hardn-logs:
  hardn-data:
```

## Log Files and Monitoring

### Log Locations
Monitor HARDN activity through log files:

- `/var/log/hardn/hardn.log` - Main application log
- `/var/log/hardn/audit.log` - Security audit log
- `/var/log/hardn/monitor.log` - Service monitoring log
- `/var/log/hardn/api.log` - API access log

### Log Analysis
```bash
# View recent activity
tail -f /var/log/hardn/hardn.log

# Search for security events
grep -i "security\|alert\|fail" /var/log/hardn/audit.log

# Monitor API access
tail -f /var/log/hardn/api.log

# Check service status changes
grep "service.*status" /var/log/hardn/monitor.log
```

## Troubleshooting

### Common Issues

#### 1. Permission Denied
```bash
# Ensure you're using sudo for privileged operations
sudo hardn setup

# Check file permissions
ls -la /usr/bin/hardn*
```

#### 2. Service Failures
```bash
# Check service logs
sudo journalctl -u hardn-monitor.service

# Restart services
sudo systemctl restart hardn-monitor.service

# Check service status
hardn monitor status
```

#### 3. Network Issues
```bash
# Check firewall rules
sudo ufw status verbose

# Verify DNS resolution
nslookup google.com

# Test network connectivity
hardn status --network
```

#### 4. Package Conflicts
```bash
# Fix dependency issues
sudo apt-get install -f

# Resolve conflicts manually
sudo apt-get remove conflicting-package

# Reinstall HARDN
sudo dpkg -i hardn_2.0.0-1_all.deb
```

### Debug Mode
```bash
# Enable debug output
hardn --verbose setup

# Debug specific operations
hardn --log-level debug monitor start

# Dry-run to see what would happen
hardn setup --dry-run --verbose
```

### Getting Help
```bash
# Show general help
hardn --help

# Show command-specific help
hardn setup --help
hardn status --help
hardn audit --help

# Check version and build info
hardn --version
```

## Best Practices

### Security Recommendations
1. **Regular Updates**: Run `hardn update` weekly
2. **Backup Before Changes**: Always backup before major changes
3. **Monitor Logs**: Regularly review security logs
4. **Test Configurations**: Use `--dry-run` for testing
5. **Automate Deployments**: Use `--non-interactive` for consistency

### Performance Optimization
1. **Schedule Scans**: Run intensive scans during off-hours
2. **Log Rotation**: Configure proper log rotation
3. **Resource Monitoring**: Monitor CPU and memory usage
4. **Network Optimization**: Optimize firewall rules

### Compliance Maintenance
1. **Regular Audits**: Schedule monthly compliance audits
2. **Documentation**: Keep security configurations documented
3. **Change Management**: Track all security changes
4. **Incident Response**: Prepare incident response procedures

## Integration Examples

### Nagios/Icinga Monitoring
```bash
# Check command for Nagios
/usr/bin/hardn status --quiet && echo "OK - HARDN running" || echo "CRITICAL - HARDN issues"
```

### Prometheus Metrics
```bash
# Start API for Prometheus scraping
hardn api --port 9090 --metrics

# Scrape configuration
curl http://localhost:9090/metrics
```

### Syslog Integration
```bash
# Configure syslog forwarding
hardn config edit

# Set syslog server
echo "syslog_server=192.168.1.100" >> /etc/hardn/hardn.conf
```

For additional support, check the [Installation Guide](install.md), [Configuration Guide](config.md), or contact the development team.