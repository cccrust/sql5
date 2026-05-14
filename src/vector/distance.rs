//! 向量距離計算
//!
//! 支援三種距離度量：
//! - L2 (歐氏距離)
//! - Cosine (餘弦距離)
//! - Hamming (漢明距離)

use crate::vector::vector::VectorType;

/// 計算 L2 歐氏距離
pub fn distance_l2(a: &VectorType, b: &VectorType) -> Result<f64, String> {
    match (a, b) {
        (VectorType::Float32(a_vec), VectorType::Float32(b_vec)) => {
            if a_vec.len() != b_vec.len() {
                return Err("Vector length mismatch".to_string());
            }
            let sum: f64 = a_vec.iter()
                .zip(b_vec.iter())
                .map(|(x, y)| (*x as f64 - *y as f64).powi(2))
                .sum();
            Ok(sum.sqrt())
        }
        (VectorType::Int8(a_vec), VectorType::Int8(b_vec)) => {
            if a_vec.len() != b_vec.len() {
                return Err("Vector length mismatch".to_string());
            }
            let sum: f64 = a_vec.iter()
                .zip(b_vec.iter())
                .map(|(&x, &y)| (x as f64 - y as f64).powi(2))
                .sum();
            Ok(sum.sqrt())
        }
        _ => Err("Cannot calculate L2 distance between bitvectors".to_string()),
    }
}

/// 計算 Cosine 距離 (1 - cosine similarity)
pub fn distance_cosine(a: &VectorType, b: &VectorType) -> Result<f64, String> {
    match (a, b) {
        (VectorType::Float32(a_vec), VectorType::Float32(b_vec)) => {
            if a_vec.len() != b_vec.len() {
                return Err("Vector length mismatch".to_string());
            }

            let dot: f64 = a_vec.iter().zip(b_vec.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
            let norm_a: f64 = a_vec.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>().sqrt();
            let norm_b: f64 = b_vec.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>().sqrt();

            if norm_a == 0.0 || norm_b == 0.0 {
                return Err("Cannot calculate cosine distance with zero vector".to_string());
            }

            let similarity = dot / (norm_a * norm_b);
            Ok(1.0 - similarity)
        }
        (VectorType::Int8(a_vec), VectorType::Int8(b_vec)) => {
            if a_vec.len() != b_vec.len() {
                return Err("Vector length mismatch".to_string());
            }

            let dot: i64 = a_vec.iter().zip(b_vec.iter())
                .map(|(&x, &y)| (x as i64) * (y as i64))
                .sum();
            let norm_a: f64 = (a_vec.iter().map(|&x| (x as i64).pow(2)).sum::<i64>() as f64).sqrt();
            let norm_b: f64 = (b_vec.iter().map(|&x| (x as i64).pow(2)).sum::<i64>() as f64).sqrt();

            if norm_a == 0.0 || norm_b == 0.0 {
                return Err("Cannot calculate cosine distance with zero vector".to_string());
            }

            let similarity = dot as f64 / (norm_a * norm_b);
            Ok(1.0 - similarity)
        }
        _ => Err("Cannot calculate cosine distance between bitvectors".to_string()),
    }
}

/// 計算 Hamming 距離 (僅適用於 bit 向量)
pub fn distance_hamming(a: &VectorType, b: &VectorType) -> Result<u32, String> {
    match (a, b) {
        (VectorType::Bit(a_vec), VectorType::Bit(b_vec)) => {
            if a_vec.len() != b_vec.len() {
                return Err("Vector length mismatch".to_string());
            }

            let mut distance = 0u32;
            for (a_byte, b_byte) in a_vec.iter().zip(b_vec.iter()) {
                let xor = a_byte ^ b_byte;
                distance += xor.count_ones();
            }

            Ok(distance)
        }
        _ => Err("Cannot calculate hamming distance between non-bit vectors".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::vector::VectorType;

    #[test]
    fn test_l2_distance() {
        let a = VectorType::Float32(vec![1.0, 1.0]);
        let b = VectorType::Float32(vec![2.0, 2.0]);
        let d = distance_l2(&a, &b).unwrap();
        assert!((d - 1.41421356237).abs() < 0.001);
    }

    #[test]
    fn test_cosine_distance() {
        let a = VectorType::Float32(vec![1.0, 0.0]);
        let b = VectorType::Float32(vec![1.0, 0.0]);
        let d = distance_cosine(&a, &b).unwrap();
        assert!(d < 0.001);
    }

    #[test]
    fn test_cosine_distance_opposite() {
        let a = VectorType::Float32(vec![1.0, 0.0]);
        let b = VectorType::Float32(vec![-1.0, 0.0]);
        let d = distance_cosine(&a, &b).unwrap();
        assert!((d - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_hamming_distance() {
        let a = VectorType::Bit(vec![0b11110000]);
        let b = VectorType::Bit(vec![0b00001111]);
        let d = distance_hamming(&a, &b).unwrap();
        assert_eq!(d, 8);
    }

    #[test]
    fn test_hamming_distance_same() {
        let a = VectorType::Bit(vec![0b11111111]);
        let b = VectorType::Bit(vec![0b11111111]);
        let d = distance_hamming(&a, &b).unwrap();
        assert_eq!(d, 0);
    }

    #[test]
    fn test_int8_l2() {
        let a = VectorType::Int8(vec![1, 2, 3]);
        let b = VectorType::Int8(vec![4, 5, 6]);
        let d = distance_l2(&a, &b).unwrap();
        assert!((d - 5.196152422706632).abs() < 0.001);
    }
}