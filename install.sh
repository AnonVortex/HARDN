#!/bin/bash

set -euo pipefail

HARDN_VERSION="2.0.0"
SOURCE_DIR="./HARDN-XDR"

error_exit() {
    echo
    echo "INSTALLATION FAILED: $1"
    echo "Please check the messages above."
    echo
    exit 1
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        error_exit "This script requires root privileges. Please run with sudo."
    fi
}

check_system() {
    if [[ ! -f /etc/debian_version ]]; then
        error_exit "This system is not Debian-based. HARDN requires Debian 12+ or Ubuntu 24.04+."
    fi
    echo "[+] OK: Debian-based system detected"
}

update_system() {
    echo "[*] Updating system packages..."
    apt update && apt upgrade -y || echo "[!] WARNING: System update encountered issues, continuing..."
}

install_dependencies() {
    echo "[*] Installing required packages..."
    apt-get update
    apt-get install -y \
        auditd audispd-plugins suricata fail2ban rkhunter chkrootkit unhide debsums lynis \
        clamav clamav-daemon clamav-freshclam yara aide aide-common rsyslog logrotate needrestart \
        apt-listchanges apt-listbugs unattended-upgrades apt-transport-https ca-certificates \
        software-properties-common lsb-release gnupg openssh-server openssh-client ufw \
        systemd-timesyncd apparmor apparmor-profiles apparmor-utils firejail \
        libpam-pwquality libpam-google-authenticator libpam-tmpdir \
        curl wget lsof psmisc procps git \
        python3-gi python3-gi-cairo python3-matplotlib python3-psutil python3-requests gir1.2-gtk-3.0 \
        debhelper-compat devscripts build-essential
}

create_system_groups() {
    echo "[*] Ensuring required system users and groups..."
    getent group systemd-network >/dev/null || groupadd -r systemd-network
    id -u systemd-network >/dev/null 2>&1 || useradd -r -M -s /usr/sbin/nologin systemd-network
    getent group systemd-journal >/dev/null || groupadd -r systemd-journal
}

install_hardn_package() {
    echo "[*] Installing HARDN from local source..."

    [[ -d "$SOURCE_DIR" ]] || error_exit "Source directory $SOURCE_DIR not found."

    pushd "$SOURCE_DIR" >/dev/null
    echo "[*] Building package in: $SOURCE_DIR"
    dpkg-buildpackage -us -uc -b || error_exit "dpkg-buildpackage failed"
    popd >/dev/null

    deb_file=$(find . -maxdepth 1 -type f -name "hardn_${HARDN_VERSION}-1_all.deb" | head -n 1)
    [[ -f "$deb_file" ]] || error_exit "Built .deb file not found"

    echo "[*] Installing package: $deb_file"
    dpkg -i "$deb_file" || apt-get install -f -y

    if ! command -v hardn >/dev/null; then
        error_exit "hardn command not found after installation"
    fi

    echo "[*] Running optional postinstall script if present..."
    [[ -x "$SOURCE_DIR/postinstall.sh" ]] && "$SOURCE_DIR/postinstall.sh"
}

show_completion() {
    cat << 'EOF'

[✓] HARDN Installation Complete

Next steps:
1. Run system hardening:
   sudo hardn setup

2. Check system status:
   hardn status

3. Run security audit:
   sudo hardn audit

4. View help:
   hardn --help

Documentation:
https://github.com/Security-International-Group/HARDN-XDR

WARNING: HARDN makes significant system changes.
         Always test in a non-production environment first.

EOF
}

main() {
    echo
    echo "HARDN v${HARDN_VERSION} Local Installer"
    echo "======================================="

    check_root
    check_system
    update_system
    create_system_groups
    install_dependencies
    install_hardn_package
    show_completion
}

main "$@"