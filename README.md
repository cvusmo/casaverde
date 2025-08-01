# Casaverde
Casaverde is a home automation tool that lets you monitor and control your household systems from a Raspberry Pi. Written in Rust with a data-oriented design, it’s built to be lightweight, secure, and efficient, even on low-resource hardware. Whether you’re toggling smart lights or keeping tabs on your greenhouse, Casaverde runs entirely on your internal network, keeping your data private and under your control. It’s a work in progress, but it’s already a practical way to bring your home’s systems together.

You can interact with Casaverde through a clean terminal user interface (TUI) or a touchscreen interface, depending on your setup. It’s designed to feel intuitive whether you’re typing commands or tapping a screen.

---

## Status: Early Development
Casaverde is actively being built, with a working TUI and touchscreen interface implemented in Rust. The server, which handles device control and data processing, can run on a Raspberry Pi or a separate PC. The client currently runs on the Pi, supporting both TUI and touchscreen modes (3.5 RPi display). I’m working toward adding iOS and Android app support for remote control within your home network.

The app uses data-oriented design and programming to keep memory and CPU usage low, making it a good fit for resource-constrained devices. Security is a priority—data stays local, and communication is designed to be private and tamper-resistant. Touchscreen support is functional but still being refined for broader display compatibility.

---

## Features
Casaverde lets you manage key household systems with a focus on simplicity and control:
- **Lighting**: Switch and tweak smart lights (like Philips Hue) with ease.
- **Greenhouse**: Keep an eye on water levels, humidity, soil moisture, and solar exposure to keep your plants happy.
- **Thermostat**: Monitor and adjust your home’s AC to stay comfortable while saving energy.
- **Energy Usage Monitoring**: Integrate with smart plugs or energy-monitoring devices to track power consumption of connected devices (e.g., lights, AC). Display usage stats and suggest optimizations (e.g., “Turning off porch lights at 10 PM could save 5 kWh/month”).
- **Casaverde UI**: Clean, simple, and easy to navigate UI for displaying all monitoring systems. Let's users turn on/off with just touch.
- **Alert Notifications**: Receive notifications for critical events.
- **Multi-Zone Support**: Allow users to gorup devices by zones (Living Room, Backyard) for easier management and monitoring. Each zone can have it's own dashboard view or automation rules.

---
## Casaverde Goals

The goal is a unified, lightweight hub for home automation that stays fast, secure, and private, all running on your internal network. All features are designed to minimize resource usage, leveraging Rust’s performance and data-oriented principles (flat data structures, minimal allocations).
- **Secure and Private**: Features avoid external dependencies, keep data local, and prioritize secure protocols (encrypted backups, sandboxed plugins).
- **User-Focused**: Suggestions enhance usability (dashboards, alerts) while staying true to the app’s internal-network-only philosophy.

---

## Follow the Journey
I’m building Casaverde step by step, sharing updates and tinkering sessions along the way. Follow along or drop by to chat:
- [Twitch](https://twitch.tv/cvusmo)
- [X](https://www.x.com/@cvusmo)

---
