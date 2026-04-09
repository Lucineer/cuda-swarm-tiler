//! cuda-swarm-tiler — GPU Monte Carlo yield simulation for MoE chips

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DieGrade { Gold, Silver, Bronze, Scrap }

impl DieGrade {
    pub fn performance(&self) -> f64 {
        match self { DieGrade::Gold => 1.0, DieGrade::Silver => 0.85, DieGrade::Bronze => 0.65, DieGrade::Scrap => 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct Die { pub x: usize, pub y: usize, pub grade: DieGrade, pub defects: u32, pub functional: bool }

#[derive(Debug)]
pub struct YieldResult {
    pub total: u32, pub gold: u32, pub silver: u32, pub bronze: u32, pub scrap: u32,
    pub gold_pct: f64, pub silver_pct: f64, pub bronze_pct: f64, pub scrap_pct: f64,
}

pub struct SwarmTiler { pub die_size_mm: f64, pub defect_rate: f64, pub wafer_mm: f64, pub repairable: u32 }

impl SwarmTiler {
    pub fn new(die_size_mm: f64, defect_rate: f64) -> Self {
        SwarmTiler { die_size_mm, defect_rate, wafer_mm: 300.0, repairable: 2 }
    }
    pub fn simulate(&self, seed: u64) -> Vec<Die> {
        let mut dies = Vec::new();
        let r = self.wafer_mm / 2.0;
        let n = (r / self.die_size_mm) as usize;
        let center = n as f64 / 2.0;
        let area_cm2 = (self.die_size_mm / 10.0).powi(2);
        let mut rng = seed;
        for yi in 0..n { for xi in 0..n {
            let cx = xi as f64 + 0.5 - center;
            let cy = yi as f64 + 0.5 - center;
            let dist = (cx * cx + cy * cy).sqrt() * self.die_size_mm;
            if dist + self.die_size_mm / 2.0 > r { continue; }
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let expected = self.defect_rate * area_cm2;
            let mut defects = 0u32; let mut l = (-expected).exp(); let mut p = 1.0;
            loop {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let u = ((rng >> 33) as f64) / (u64::MAX as f64 / 2.0 + 1.0);
                p *= u; if p < l { break; } defects += 1;
            }
            let zone = dist / r;
            let grade = if defects == 0 { DieGrade::Gold }
                else if defects <= self.repairable && zone < 0.85 { DieGrade::Silver }
                else if defects <= self.repairable + 1 && zone < 0.7 { DieGrade::Bronze }
                else { DieGrade::Scrap };
            dies.push(Die { x: xi, y: yi, grade, defects, functional: grade != DieGrade::Scrap });
        }}
        dies
    }
    pub fn yield_analysis(&self, trials: u32) -> YieldResult {
        let (mut tg, mut ts, mut tb, mut tsc, mut tt) = (0u32,0,0,0,0);
        for t in 0..trials { for d in self.simulate(t as u64) {
            tt += 1; match d.grade { DieGrade::Gold => tg+=1, DieGrade::Silver => ts+=1, DieGrade::Bronze => tb+=1, DieGrade::Scrap => tsc+=1 }
        }}
        let t = tt as f64;
        YieldResult { total: tt, gold: tg, silver: ts, bronze: tb, scrap: tsc,
            gold_pct: tg as f64/t*100.0, silver_pct: ts as f64/t*100.0, bronze_pct: tb as f64/t*100.0, scrap_pct: tsc as f64/t*100.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_simulate() { let t = SwarmTiler::new(20.0, 0.05); let d = t.simulate(42); assert!(d.len() > 50); }
    #[test] fn test_yield() { let t = SwarmTiler::new(20.0, 0.05); let y = t.yield_analysis(5); assert!(y.gold_pct > 0.0); }
}
