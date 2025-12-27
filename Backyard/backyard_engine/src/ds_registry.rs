// src/ds_registry.rs
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DsLifeState {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct DsInstance {
    pub ds_id: String,

    // DS가 게임을 열어둔 주소/포트(클라가 실제로 접속할 곳)
    pub public_ip: std::net::IpAddr,
    pub game_port: u16,

    pub max_players: u16,
    pub current_players: u16,
    pub state: u8, // DS가 보내는 state (0/1/2/3...)

    pub build: String,

    // 관리용
    pub tcp_peer: SocketAddr,     // Manager와 연결된 TCP peer
    pub token: mio::Token,        // 연결 token
    pub last_seen: Instant,
    pub life: DsLifeState,
}

#[derive(Debug)]
pub struct DsRegistry {
    by_id: HashMap<String, DsInstance>,
    token_to_id: HashMap<mio::Token, String>,
    timeout: Duration,
}

impl DsRegistry {
    pub fn new(timeout: Duration) -> Self {
        Self {
            by_id: HashMap::new(),
            token_to_id: HashMap::new(),
            timeout,
        }
    }

    pub fn on_register(
        &mut self,
        now: Instant,
        token: mio::Token,
        tcp_peer: SocketAddr,
        ds_id: String,
        game_port: u16,
        max_players: u16,
        build: String,
    ) {
        let public_ip = tcp_peer.ip(); // 1차: tcp peer ip를 public ip로 사용(나중에 개선 가능)

        let inst = DsInstance {
            ds_id: ds_id.clone(),
            public_ip,
            game_port,
            max_players,
            current_players: 0,
            state: 0,
            build,
            tcp_peer,
            token,
            last_seen: now,
            life: DsLifeState::Up,
        };

        self.by_id.insert(ds_id.clone(), inst);
        self.token_to_id.insert(token, ds_id.clone());

        println!(
            "[Registry] DS REGISTERED id={} {}:{} max={}",
            ds_id, public_ip, game_port, max_players
        );
    }

    pub fn on_heartbeat(
        &mut self,
        now: Instant,
        ds_id: &str,
        current_players: u16,
        state: u8,
    ) {
        if let Some(ds) = self.by_id.get_mut(ds_id) {
            ds.current_players = current_players;
            ds.state = state;
            ds.last_seen = now;
            ds.life = DsLifeState::Up;
        } else {
            // 아직 register 안 된 ds_id로 heartbeat가 오면 로그만
            println!("[Registry] heartbeat from unknown ds_id={}", ds_id);
        }
    }

    pub fn on_disconnect(&mut self, token: mio::Token) {
        if let Some(ds_id) = self.token_to_id.remove(&token) {
            if let Some(ds) = self.by_id.get_mut(&ds_id) {
                ds.life = DsLifeState::Down;
                println!("[Registry] DS DOWN (disconnect) id={}", ds_id);
            }
        }
    }

    pub fn reap_timeouts(&mut self, now: Instant) {
        let timeout = self.timeout;

        for (id, ds) in self.by_id.iter_mut() {
            if ds.life == DsLifeState::Up {
                let elapsed = now.duration_since(ds.last_seen);
                if elapsed > timeout {
                    ds.life = DsLifeState::Down;
                    println!("[Registry] DS DOWN (timeout) id={} elapsed={:?}", id, elapsed);
                }
            }
        }
    }

    // 디버깅용(선택)
    pub fn dump(&self) {
        println!("=== DS Registry dump ===");
        for (id, ds) in self.by_id.iter() {
            println!(
                "id={} life={:?} {}:{} players={}/{} state={} last_seen={:?} build={}",
                id, ds.life, ds.public_ip, ds.game_port,
                ds.current_players, ds.max_players, ds.state,
                ds.last_seen.elapsed(), ds.build
            );
        }
    }
}
