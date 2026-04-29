use iced::widget::{Column, button, column, text};
use netstat2::*;

#[derive(Default)]
struct Harbor {
    value: String,
}

#[derive(Clone, Copy)]
enum Message {
    FetchPorts,
}

impl Harbor {
    fn update(&mut self, message: Message) {
        match message {
            Message::FetchPorts => {
                self.fetch_ports();
            }
        }
    }

    fn fetch_ports(&mut self) {
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
        let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap_or_default();

        self.value.clear();
        for si in sockets_info {
            match si.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp) => {
                    self.value.push_str(&format!(
                        "TCP: {}:{} -> {}:{} {:?} {:?}\n",
                        tcp.local_addr,
                        tcp.local_port,
                        tcp.remote_addr,
                        tcp.remote_port,
                        si.associated_pids,
                        tcp.state,
                    ));
                }

                ProtocolSocketInfo::Udp(udp_si) => {
                    self.value.push_str(&format!(
                        "UDP {}:{} -> *:* {:?}\n",
                        udp_si.local_addr, udp_si.local_port, si.associated_pids,
                    ));
                }
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            column![
                text("HARBOR v0.1").size(50),
                text("Manage your system ports in one place"),
                text("by Codemesh <codemesh4@gmail.com>"),
            ],
            column![
                text(&self.value),
                button("Fetch Ports").on_press(Message::FetchPorts),
            ]
        ]
    }
}

fn main() -> iced::Result {
    iced::run(Harbor::update, Harbor::view)
}

#[test]
fn test_port_fetch() {
    let mut harbor = Harbor::default();
    harbor.fetch_ports();
    // Just verify we got some output if ports are open
    assert!(!harbor.value.is_empty());
}
