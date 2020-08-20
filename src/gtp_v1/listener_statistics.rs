use std::fmt;

#[derive(Copy, Clone)]
pub struct Statistics {
    rx_gtp: usize,
    rx_ignored_gtp: usize,
    rx_gtp_echo_request: usize,
    tx_gtp_echo_request: usize,
    tx_gtp_echo_response: usize,
    rx_gtp_echo_response: usize,
    tx_gtp: usize,
    rx_ip: usize,
    tx_ip: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            rx_gtp: 0,
            rx_ignored_gtp: 0,
            tx_gtp: 0,
            rx_gtp_echo_request: 0,
            tx_gtp_echo_request: 0,
            tx_gtp_echo_response: 0,
            rx_gtp_echo_response: 0,
            rx_ip: 0,
            tx_ip: 0,
        }
    }

    pub fn rx_gtp_add(&mut self, n: usize) -> usize {
        self.rx_gtp = self.rx_gtp + n;

        self.rx_gtp
    }

    pub fn rx_gtp_echo_request_add(&mut self, n: usize) -> usize {
        self.rx_gtp_echo_request = self.rx_gtp_echo_request + n;

        self.rx_gtp_echo_request
    }

    pub fn rx_gtp_echo_response_add(&mut self, n: usize) -> usize {
        self.rx_gtp_echo_response = self.rx_gtp_echo_response + n;

        self.rx_gtp_echo_response
    }

    pub fn tx_gtp_echo_request_add(&mut self, n: usize) -> usize {
        self.tx_gtp_echo_request = self.tx_gtp_echo_request + n;

        self.tx_gtp_echo_request
    }

    pub fn tx_gtp_echo_response_add(&mut self, n: usize) -> usize {
        self.tx_gtp_echo_response = self.tx_gtp_echo_response + n;

        self.tx_gtp_echo_response
    }

    pub fn rx_ignored_gtp_add(&mut self, n: usize) -> usize {
        self.rx_ignored_gtp = self.rx_ignored_gtp + n;

        self.rx_ignored_gtp
    }

    pub fn tx_gtp_add(&mut self, n: usize) -> usize {
        self.tx_gtp = self.tx_gtp + n;

        self.tx_gtp
    }

    pub fn rx_ip_add(&mut self, n: usize) -> usize {
        self.rx_ip = self.rx_ip + n;

        self.rx_ip
    }

    pub fn tx_ip_add(&mut self, n: usize) -> usize {
        self.tx_ip = self.tx_ip + n;

        self.tx_ip
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(
            f, 
            "Tunnel Stats\n==========================\nGTP Packets: Rx: {} Rx Ingored: {} Tx: {}\nIP Packets: Rx: {} Tx: {}\n==========================",
            self.rx_gtp, self.rx_ignored_gtp, self.tx_gtp,
            self.rx_ip, self.tx_ip
        )
    }
}