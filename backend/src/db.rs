use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::Client;

#[derive(Debug, Serialize)]
pub struct SystemSnapshot {
    pub timestamp: i64,
    pub node_name: String,
    pub cpu_usage_percent: f64,
    pub cpu_core_count: i64,
    pub mem_total_bytes: i64,
    pub mem_used_bytes: i64,
    pub mem_available_bytes: i64,
    pub swap_total_bytes: i64,
    pub swap_used_bytes: i64,
}

#[derive(Debug, Serialize)]
pub struct CpuCoreSnapshot {
    pub timestamp: i64,
    pub node_name: String,
    pub core_id: i64,
    pub usage_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct DiskSnapshot {
    pub timestamp: i64,
    pub node_name: String,
    pub name: String,
    pub mount_point: String,
    pub total_bytes: i64,
    pub used_bytes: i64,
    pub available_bytes: i64,
    pub usage_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub system: Vec<SystemSnapshot>,
    pub cores: Vec<CpuCoreSnapshot>,
    pub disks: Vec<DiskSnapshot>,
}

// QuestDB sends TIMESTAMP as OID 1114 (PostgreSQL timestamp); tokio-postgres maps it to SystemTime
fn ts_to_millis(ts: SystemTime) -> i64 {
    ts.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[derive(Debug, Serialize)]
pub struct ClusterHistoryPoint {
    pub timestamp: i64,
    pub cpu_usage_percent: f64,
    pub mem_used_bytes: i64,
    pub mem_total_bytes: i64,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub cluster: Vec<ClusterHistoryPoint>,
}

pub async fn query_history(db: &Client) -> Result<HistoryResponse, tokio_postgres::Error> {
    let rows = db
        .query(
            "SELECT timestamp, avg(cpu_usage_percent) as cpu, \
             sum(mem_used_bytes) as mem_used, sum(mem_total_bytes) as mem_total \
             FROM system_metrics \
             WHERE timestamp > dateadd('h', -1, now()) \
             SAMPLE BY 10s FILL(PREV)",
            &[],
        )
        .await
        .unwrap_or_default();

    let cluster = rows
        .iter()
        .map(|row| ClusterHistoryPoint {
            timestamp: ts_to_millis(row.get::<_, SystemTime>(0)),
            cpu_usage_percent: row.get::<_, f64>(1),
            mem_used_bytes: row.get::<_, i64>(2),
            mem_total_bytes: row.get::<_, i64>(3),
        })
        .collect();

    Ok(HistoryResponse { cluster })
}

pub async fn query_latest_metrics(db: &Client) -> Result<MetricsResponse, tokio_postgres::Error> {
    let system_rows = db
        .query(
            "SELECT timestamp, node_name, cpu_usage_percent, cpu_core_count, \
             mem_total_bytes, mem_used_bytes, mem_available_bytes, \
             swap_total_bytes, swap_used_bytes \
             FROM system_metrics \
             LATEST ON timestamp PARTITION BY node_name",
            &[],
        )
        .await
        .unwrap_or_default();

    let system = system_rows
        .iter()
        .map(|row| SystemSnapshot {
            timestamp: ts_to_millis(row.get::<_, SystemTime>(0)),
            node_name: row.get(1),
            cpu_usage_percent: row.get::<_, f64>(2),
            cpu_core_count: row.get::<_, i64>(3),
            mem_total_bytes: row.get::<_, i64>(4),
            mem_used_bytes: row.get::<_, i64>(5),
            mem_available_bytes: row.get::<_, i64>(6),
            swap_total_bytes: row.get::<_, i64>(7),
            swap_used_bytes: row.get::<_, i64>(8),
        })
        .collect();

    let core_rows = db
        .query(
            "SELECT timestamp, node_name, core_id, usage_percent \
             FROM cpu_core_metrics \
             LATEST ON timestamp PARTITION BY node_name, core_id",
            &[],
        )
        .await
        .unwrap_or_default();

    let cores = core_rows
        .iter()
        .map(|row| CpuCoreSnapshot {
            timestamp: ts_to_millis(row.get::<_, SystemTime>(0)),
            node_name: row.get(1),
            core_id: row.get::<_, i64>(2),
            usage_percent: row.get::<_, f64>(3),
        })
        .collect();

    let disk_rows = db
        .query(
            "SELECT timestamp, node_name, name, mount_point, \
             total_bytes, used_bytes, available_bytes, usage_percent \
             FROM disk_metrics \
             LATEST ON timestamp PARTITION BY node_name, name",
            &[],
        )
        .await
        .unwrap_or_default();

    let disks = disk_rows
        .iter()
        .map(|row| DiskSnapshot {
            timestamp: ts_to_millis(row.get::<_, SystemTime>(0)),
            node_name: row.get(1),
            name: row.get(2),
            mount_point: row.get(3),
            total_bytes: row.get::<_, i64>(4),
            used_bytes: row.get::<_, i64>(5),
            available_bytes: row.get::<_, i64>(6),
            usage_percent: row.get::<_, f64>(7),
        })
        .collect();

    Ok(MetricsResponse {
        system,
        cores,
        disks,
    })
}
