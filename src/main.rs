use tokio::time::Duration;
use tokio::sync::broadcast;
use tokio;

#[derive(Debug, Clone)]
struct Signal {
    x: u32,
    y: u32,
    strength: u32,
}

#[derive(Debug, Clone)]
struct Antenna {
    id: u32,
    x: u32,
    y: u32,
    radius: u32,
}

fn antenna_detects(antenna: &Antenna, signal: &Signal) -> bool {
    let dx = (antenna.x as i32 - signal.x as i32) as f32;
    let dy = (antenna.y as i32 - signal.y as i32) as f32;
    let dist = (dx * dx + dy * dy).sqrt();
    dist <= (antenna.radius + signal.strength) as f32
}

#[tokio::main] // ✅ async main
async fn main() {
    println!("🚀 Симуляция антенн запущена");

    // Создаём broadcast-канал
    let (tx, rx) = broadcast::channel(100);

    let antennas = vec![
        Antenna { id: 1, x: 10, y: 10, radius: 5 },
        Antenna { id: 2, x: 20, y: 20, radius: 6 },
        Antenna { id: 3, x: 30, y: 30, radius: 4 },
    ];

    // --- Задача: генератор сигналов ---
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        let mut id = 0;
        loop {
            interval.tick().await;
            id += 1;
            let signal = Signal {
                x: 15 + id * 3 % 20,
                y: 15 + id * 7 % 20,
                strength: 4,
            };
            println!("🌌 Сигнал {}: ({}, {}) r={}", id, signal.x, signal.y, signal.strength);
            let _ = tx_clone.send(signal); // send() не нужен await
        }
    });

    // --- Задачи: антенны ---
    for antenna in antennas {
        let mut rx = rx.resubscribe(); // ✅ Работает!
        tokio::spawn(async move {
            while let Ok(signal) = rx.recv().await {
                if antenna_detects(&antenna, &signal) {
                    println!("✅ Антенна {} ПОЙМАЛА сигнал!", antenna.id);
                }
            }
        });
    }

    // Главный поток ждёт вечно
    tokio::time::sleep(Duration::from_secs(30)).await;
    println!("Симуляция завершена.");
}
