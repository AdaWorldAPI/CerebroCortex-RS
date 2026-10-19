use chrono::{DateTime, Utc};

use crate::config::{ACTR_DECAY_RATE, ACTR_MIN_TIME_SECONDS};

/// B(t) = ln( Σ t_k^{-d} )
///
/// Matches Python `base_level_activation()` in `activation/strength.py` exactly.
/// NOTE: no noise here — noise (s parameter) lives in `recall_probability()`.
pub fn base_level_activation(
    access_times: &[DateTime<Utc>],
    now: DateTime<Utc>,
    decay: f32,
) -> f32 {
    if access_times.is_empty() {
        return f32::NEG_INFINITY;
    }
    let sum: f32 = access_times
        .iter()
        .map(|t| {
            let secs = (now - *t).num_seconds() as f32;
            secs.max(ACTR_MIN_TIME_SECONDS).powf(-decay)
        })
        .sum();
    if sum <= 0.0 {
        return f32::NEG_INFINITY;
    }
    sum.ln()
}

/// Convenience wrapper using default decay from config.
pub fn actr_activation(access_times: &[DateTime<Utc>], now: DateTime<Utc>) -> f32 {
    base_level_activation(access_times, now, ACTR_DECAY_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn now_fixed() -> DateTime<Utc> {
        // 2025-01-01T12:00:00Z = Unix 1735732800
        chrono::DateTime::from_timestamp(1_735_732_800, 0).unwrap()
    }

    #[test]
    fn single_1s_access_is_zero() {
        let now = now_fixed();
        let times = vec![now - Duration::seconds(1)];
        let b = base_level_activation(&times, now, 0.5);
        // B = ln(1^{-0.5}) = ln(1) = 0.0
        assert!((b - 0.0).abs() < 1e-5, "got {b}");
    }

    #[test]
    fn single_60s_access() {
        let now = now_fixed();
        let times = vec![now - Duration::seconds(60)];
        let b = base_level_activation(&times, now, 0.5);
        let expected = -2.047_172_3_f32;
        assert!((b - expected).abs() < 1e-4, "got {b}, expected {expected}");
    }

    #[test]
    fn empty_access_is_neg_inf() {
        let b = actr_activation(&[], chrono::Utc::now());
        assert_eq!(b, f32::NEG_INFINITY);
    }

    #[test]
    fn more_recent_access_wins() {
        let now = now_fixed();
        let recent = vec![now - Duration::seconds(10)];
        let stale  = vec![now - Duration::seconds(86400 * 30)];
        assert!(
            base_level_activation(&recent, now, 0.5) >
            base_level_activation(&stale,  now, 0.5)
        );
    }

    #[test]
    fn three_access_case() {
        let now = now_fixed();
        let times = vec![
            now - Duration::seconds(60),
            now - Duration::seconds(3600),
            now - Duration::seconds(86400),
        ];
        let b = base_level_activation(&times, now, 0.5);
        let expected = -1.902_680_9_f32;
        assert!((b - expected).abs() < 1e-4, "got {b}, expected {expected}");
    }
    // P0: deterministic replay contracts before any SoA backend substitution.
    #[test]
    fn replay_is_independent_of_access_order() {
        let now = now_fixed();
        let chronological = vec![
            now - Duration::seconds(86_400),
            now - Duration::seconds(3_600),
            now - Duration::seconds(60),
        ];
        let mut reversed = chronological.clone();
        reversed.reverse();
        let a = base_level_activation(&chronological, now, 0.5);
        let b = base_level_activation(&reversed, now, 0.5);
        assert!((a - b).abs() < 1e-5, "access order changed activation: {a} vs {b}");
    }

    #[test]
    fn replay_clamps_zero_and_future_access_to_minimum_age() {
        let now = now_fixed();
        let at_now = base_level_activation(&[now], now, 0.5);
        let in_future = base_level_activation(&[now + Duration::seconds(30)], now, 0.5);
        assert!(at_now.is_finite());
        assert_eq!(at_now, in_future, "minimum-age clamp changed");
    }

    #[test]
    fn replay_more_accesses_increase_activation() {
        let now = now_fixed();
        let one = vec![now - Duration::seconds(60)];
        let two = vec![one[0], now - Duration::seconds(3_600)];
        assert!(base_level_activation(&two, now, 0.5) > base_level_activation(&one, now, 0.5));
    }

}
