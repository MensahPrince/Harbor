Readme# Accessing System Ports in an Iced App (Harbor)

For a port manager app, you'll need two things: **fetching system port data** and **wiring it into Iced's architecture** via Commands or Subscriptions.

---

## 1. Getting System Port Data

The cleanest approach in Rust is the `netstat2` or `sysinfo` crate, or parsing `/proc/net/tcp` directly (Linux). The most cross-platform option:

```toml
# Cargo.toml
[dependencies]
iced = { version = "0.13", features = ["tokio"] }
netstat2 = "0.9"
sysinfo = "0.30"
```

```rust
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

pub struct PortEntry {
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub local_addr: String,
    pub local_port: u16,
    pub state: String,
    pub protocol: String,
}

pub fn fetch_ports() -> Vec<PortEntry> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let sockets = get_sockets_info(af_flags, proto_flags).unwrap_or_default();
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    sockets
        .into_iter()
        .map(|sock| {
            let (local_port, state, protocol) = match &sock.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp) => (
                    tcp.local_port,
                    format!("{:?}", tcp.state),
                    "TCP".to_string(),
                ),
                ProtocolSocketInfo::Udp(udp) => (udp.local_port, "—".to_string(), "UDP".to_string()),
            };

            let pid = sock.associated_pids.first().copied();
            let process_name = pid.and_then(|p| {
                sys.process(sysinfo::Pid::from(p as usize))
                    .map(|proc| proc.name().to_string_lossy().to_string())
            });

            PortEntry {
                pid,
                process_name,
                local_addr: "127.0.0.1".to_string(), // extract from sock
                local_port,
                state,
                protocol,
            }
        })
        .collect()
}
```

---

## 2. Integrating with Iced (TEA Pattern)

In Iced, you fetch data via **`Command`** (one-shot) and refresh via **`Subscription`** (polling).

```rust
use iced::{Application, Command, Element, Subscription, Theme};
use iced::time;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {
    PortsLoaded(Vec<PortEntry>),
    Refresh,
    Kill(u32), // kill by PID
}

pub struct Harbor {
    ports: Vec<PortEntry>,
    loading: bool,
}

impl Application for Harbor {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { ports: vec![], loading: true },
            // Load ports on startup
            Command::perform(
                async { fetch_ports() },
                Message::PortsLoaded,
            ),
        )
    }

    fn title(&self) -> String {
        "Harbor — Port Manager".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PortsLoaded(ports) => {
                self.ports = ports;
                self.loading = false;
                Command::none()
            }
            Message::Refresh => {
                self.loading = true;
                Command::perform(async { fetch_ports() }, Message::PortsLoaded)
            }
            Message::Kill(pid) => {
                // send SIGKILL or use sysinfo to kill
                Command::none()
            }
        }
    }

    // Auto-refresh every 3 seconds
    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(3)).map(|_| Message::Refresh)
    }

    fn view(&self) -> Element<Message> {
        // Your Iced UI here
        todo!()
    }
}
```

---

## 3. Key Points

| Concern | Approach |
|---|---|
| **Fetch ports** | `netstat2::get_sockets_info()` |
| **Process names** | `sysinfo::System::process(pid)` |
| **One-time load** | `Command::perform(async { ... }, Msg)` |
| **Auto-refresh** | `Subscription` with `time::every(Duration)` |
| **Kill a process** | `sysinfo::Process::kill()` or `nix::sys::signal` |

---

## Permissions Note (Linux/macOS)

Reading ports owned by other users requires **root** or `CAP_NET_ADMIN`. You can either:
- Run Harbor with `sudo` for full visibility
- Or use `pkexec` / polkit to elevate just the fetch call



For Harbor (port manager), you'll want to poll system information periodically rather than relying on manual refreshes.

