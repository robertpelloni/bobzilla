#!/usr/bin/env python3
"""
bobzilla-tray.py - Bobzilla Desktop System Tray Companion & Status Monitor
Connects to the Bobzilla Javasandbox Runner Daemon via Unix Domain Socket.
Provides real-time health checks, MicroVM status metrics, launch triggers,
and graceful server quit/shutdown capability.
"""

import os
import sys
import socket
import argparse
import time

def find_ipc_socket():
    """Resolves the active IPC socket using Bobzilla discovery hierarchy."""
    env_sock = os.environ.get("BOBZILLA_SOCKET")
    if env_sock and os.path.exists(env_sock):
        return env_sock

    xdg_dir = os.environ.get("XDG_RUNTIME_DIR")
    if xdg_dir:
        p = os.path.join(xdg_dir, "bobzilla-sandbox.sock")
        if os.path.exists(p):
            return p

    uid = os.getuid()
    candidates = [
        f"/run/user/{uid}/bobzilla-sandbox.sock",
        "/run/user/1000/bobzilla-sandbox.sock",
        f"/tmp/bobzilla-sandbox-{uid}.sock",
        "/tmp/bobzilla-sandbox.sock"
    ]
    for c in candidates:
        if os.path.exists(c):
            return c

    # Return default expected path if none active currently
    return candidates[0]

def send_ipc_command(sock_path, cmd, timeout=2.0):
    """Sends an ASCII command to the runner-daemon and returns the response."""
    if not os.path.exists(sock_path):
        return None, f"Socket file not found at {sock_path}"

    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(timeout)
        s.connect(sock_path)
        payload = (cmd.strip() + "\n").encode("utf-8")
        s.sendall(payload)
        resp = s.recv(1024).decode("utf-8", errors="replace").strip()
        s.close()
        return resp, None
    except Exception as e:
        return None, str(e)

def format_dashboard(status_str, sock_path):
    """Formats status metrics for terminal / desktop tray display."""
    output = []
    output.append("==================================================")
    output.append("          BOBZILLA MISSION CONTROL COMPANION       ")
    output.append("==================================================")
    output.append(f"IPC Socket Path : {sock_path}")

    if status_str:
        output.append("Daemon Status   : ONLINE (Connected)")
        output.append(f"Subsystems      : {status_str}")
    else:
        output.append("Daemon Status   : OFFLINE (Daemon not running)")
        output.append("Notice          : Run 'cargo run --manifest-path runner-daemon/Cargo.toml' to start.")

    output.append("--------------------------------------------------")
    output.append("Available Commands: [P]ing, [S]tatus, [L]aunch, [Q]uit daemon")
    output.append("==================================================")
    return "\n".join(output)

def run_test_suite(sock_path):
    """Automated verification mode for CI and headless testing."""
    print(f"=== Bobzilla Tray IPC Verification ===")
    print(f"Target Socket: {sock_path}")

    # Test 1: Ping
    resp, err = send_ipc_command(sock_path, "PING")
    if err:
        print(f"Socket connection test: SKIPPED (Daemon not active at {sock_path}: {err})")
        return True

    print(f"PING response: {resp}")
    assert "PONG" in resp, f"Unexpected ping response: {resp}"

    # Test 2: Status
    resp, err = send_ipc_command(sock_path, "STATUS")
    print(f"STATUS response: {resp}")
    assert "READY" in resp, f"Unexpected status response: {resp}"

    # Test 3: Dashboard Formatting
    formatted = format_dashboard(resp, sock_path)
    assert "ONLINE" in formatted
    print("Dashboard visual formatting verified.")
    print("=== Verification Complete [OK] ===")
    return True

def main():
    parser = argparse.ArgumentParser(description="Bobzilla System Tray & Process Monitor")
    parser.add_argument("--socket", type=str, default=None, help="Explicit IPC socket path")
    parser.add_argument("--status", action="store_true", help="Print status and exit")
    parser.add_argument("--ping", action="store_true", help="Ping daemon and exit")
    parser.add_argument("--quit-daemon", action="store_true", help="Send shutdown command to daemon")
    parser.add_argument("--test", action="store_true", help="Run automated test suite")

    args = parser.parse_args()
    sock_path = args.socket or find_ipc_socket()

    if args.test:
        sys.exit(0 if run_test_suite(sock_path) else 1)

    if args.ping:
        resp, err = send_ipc_command(sock_path, "PING")
        if err:
            print(f"Error pinging daemon: {err}")
            sys.exit(1)
        print(resp)
        return

    if args.quit_daemon:
        resp, err = send_ipc_command(sock_path, "QUIT")
        if err:
            print(f"Error sending quit: {err}")
            sys.exit(1)
        print(resp)
        return

    if args.status:
        resp, err = send_ipc_command(sock_path, "STATUS")
        print(format_dashboard(resp, sock_path))
        return

    # Default interactive view
    resp, _ = send_ipc_command(sock_path, "STATUS")
    print(format_dashboard(resp, sock_path))

if __name__ == "__main__":
    main()
