//! 向量 SQL 函數
//!
//! 對應 sqlite-vec 的 API：
//! - vec_f32(vector) - 建立 float32 向量
//! - vec_int8(vector) - 建立 int8 向量
//! - vec_bit(vector) - 建立 bit 向量
//! - vec_length(vector) - 向量維度
//! - vec_type(vector) - 向量類型
//! - vec_to_json(vector) - 轉 JSON
//! - vec_normalize(vector) - L2 正規化
//! - vec_add(a, b) - 向量相加
//! - vec_sub(a, b) - 向量相減
//! - vec_slice(vector, start, end) - 擷取子向量
//! - vec_distance_L2(a, b) - L2 距離
//! - vec_distance_cosine(a, b) - Cosine 距離
//! - vec_distance_hamming(a, b) - Hamming 距離
//! - vec_quantize_binary(vector) - 二值化量化

use crate::vector::vector::{VectorType, parse_dimension_type};
use crate::vector::distance::{distance_l2, distance_cosine, distance_hamming};

/// vec_f32 - 建立 float32 向量
pub fn vec_f32(value: &str) -> Result<String, String> {
    let vector = VectorType::from_json(value, "float32")?;
    Ok(vector.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_int8 - 建立 int8 向量
pub fn vec_int8(value: &str) -> Result<String, String> {
    let vector = VectorType::from_json(value, "int8")?;
    Ok(vector.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_bit - 建立 bit 向量
pub fn vec_bit(value: &str) -> Result<String, String> {
    let vector = VectorType::from_json(value, "bit")?;
    Ok(vector.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_length - 向量維度
pub fn vec_length(value: &str) -> Result<usize, String> {
    // 嘗試解析為 JSON 向量
    if value.starts_with('[') {
        let vector = VectorType::from_json(value, "float32")?;
        return Ok(vector.dimension());
    }

    // 嘗試解析為 BLOB (hex string)
    if value.starts_with("0x") || value.starts_with("0X") {
        let hex = &value[2..];
        let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;

        // 自動偵測類型
        if bytes.len() % 4 == 0 {
            let vector = VectorType::from_blob(&bytes, "float32")?;
            return Ok(vector.dimension());
        } else {
            let vector = VectorType::from_blob(&bytes, "int8")?;
            return Ok(vector.dimension());
        }
    }

    // 嘗試識別前綴
    if value.len() >= 2 && value.len() <= 4 {
        let subtype = value.as_bytes()[0];
        if subtype == 0xDF { // 223 = float32
            let vector = VectorType::from_blob(&value.as_bytes()[1..], "float32")?;
            return Ok(vector.dimension());
        } else if subtype == 0xE1 { // 225 = int8
            let vector = VectorType::from_blob(&value.as_bytes()[1..], "int8")?;
            return Ok(vector.dimension());
        }
    }

    Err("Unknown vector format".to_string())
}

/// vec_type - 向量類型
pub fn vec_type(value: &str) -> Result<&'static str, String> {
    if value.starts_with('[') {
        // JSON 格式無法直接判斷類型，回傳預設值
        if value.contains('.') {
            return Ok("float32");
        } else {
            return Ok("int8");
        }
    }

    // 檢查前綴
    if value.len() >= 2 {
        let first_byte = value.as_bytes()[0];
        if first_byte == 0xDF { return Ok("float32"); } // 223
        if first_byte == 0xE1 { return Ok("int8"); }   // 225
        if first_byte == 0xE0 { return Ok("bit"); }    // 224
    }

    Ok("unknown")
}

/// vec_to_json - 轉 JSON
pub fn vec_to_json(value: &str) -> Result<String, String> {
    if value.starts_with('[') {
        // 已經是 JSON
        let vector = VectorType::from_json(value, "float32")?;
        return Ok(vector.to_json());
    }

    // BLOB 格式
    let hex = if value.starts_with("0x") || value.starts_with("0X") {
        &value[2..]
    } else {
        value
    };

    let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;

    // 自動偵測類型
    if bytes.len() % 4 == 0 {
        let vector = VectorType::from_blob(&bytes, "float32")?;
        Ok(vector.to_json())
    } else if bytes.len() <= 128 {
        let vector = VectorType::from_blob(&bytes, "int8")?;
        Ok(vector.to_json())
    } else {
        let vector = VectorType::from_blob(&bytes, "bit")?;
        Ok(vector.to_json())
    }
}

/// vec_normalize - L2 正規化
pub fn vec_normalize(value: &str) -> Result<String, String> {
    let vector = if value.starts_with('[') {
        VectorType::from_json(value, "float32")?
    } else {
        let hex = if value.starts_with("0x") || value.starts_with("0X") {
            &value[2..]
        } else {
            value
        };
        let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;
        VectorType::from_blob(&bytes, "float32")?
    };

    let normalized = vector.normalize()?;
    Ok(normalized.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_add - 向量相加
pub fn vec_add(a: &str, b: &str) -> Result<String, String> {
    let vec_a = parse_vector(a)?;
    let vec_b = parse_vector(b)?;
    let result = vec_a.add(&vec_b)?;
    Ok(result.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_sub - 向量相減
pub fn vec_sub(a: &str, b: &str) -> Result<String, String> {
    let vec_a = parse_vector(a)?;
    let vec_b = parse_vector(b)?;
    let result = vec_a.sub(&vec_b)?;
    Ok(result.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_slice - 擷取子向量
pub fn vec_slice(value: &str, start: usize, end: usize) -> Result<String, String> {
    let vector = parse_vector(value)?;
    let sliced = vector.slice(start, end)?;
    Ok(sliced.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// vec_distance_L2 - L2 距離
pub fn vec_distance_l2(a: &str, b: &str) -> Result<f64, String> {
    let vec_a = parse_vector(a)?;
    let vec_b = parse_vector(b)?;
    distance_l2(&vec_a, &vec_b)
}

/// vec_distance_cosine - Cosine 距離
pub fn vec_distance_cosine(a: &str, b: &str) -> Result<f64, String> {
    let vec_a = parse_vector(a)?;
    let vec_b = parse_vector(b)?;
    distance_cosine(&vec_a, &vec_b)
}

/// vec_distance_hamming - Hamming 距離
pub fn vec_distance_hamming(a: &str, b: &str) -> Result<u32, String> {
    let vec_a = parse_vector(a)?;
    let vec_b = parse_vector(b)?;
    distance_hamming(&vec_a, &vec_b)
}

/// vec_quantize_binary - 二值化量化
pub fn vec_quantize_binary(value: &str) -> Result<String, String> {
    let vector = parse_vector(value)?;
    let quantized = vector.quantize_binary()?;
    Ok(quantized.to_blob().iter().map(|b| format!("{:02x}", b)).collect())
}

/// 輔助函數：解析向量字串
fn parse_vector(value: &str) -> Result<VectorType, String> {
    let value = value.trim();

    if value.starts_with('[') {
        // 自動偵測類型
        if value.contains('.') {
            VectorType::from_json(value, "float32")
        } else {
            VectorType::from_json(value, "int8")
        }
    } else {
        let hex = if value.starts_with("0x") || value.starts_with("0X") {
            &value[2..]
        } else {
            value
        };
        let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;

        if bytes.len() % 4 == 0 {
            VectorType::from_blob(&bytes, "float32")
        } else if bytes.len() <= 128 {
            VectorType::from_blob(&bytes, "int8")
        } else {
            VectorType::from_blob(&bytes, "bit")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_f32() {
        let result = vec_f32("[1.0, 2.0, 3.0]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_vec_length() {
        let len = vec_length("[1.0, 2.0, 3.0]").unwrap();
        assert_eq!(len, 3);
    }

    #[test]
    fn test_vec_to_json() {
        let json = vec_to_json("[1.0, 2.0, 3.0]").unwrap();
        assert!(json.contains("1.000000"));
    }

    #[test]
    fn test_vec_normalize() {
        let result = vec_normalize("[2.0, 0.0, 0.0]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_vec_add() {
        let result = vec_add("[1.0, 2.0]", "[3.0, 4.0]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_vec_distance_l2() {
        let d = vec_distance_l2("[1.0, 1.0]", "[2.0, 2.0]").unwrap();
        assert!((d - 1.41421356237).abs() < 0.001);
    }

    #[test]
    fn test_vec_distance_cosine() {
        let d = vec_distance_cosine("[1.0, 0.0]", "[1.0, 0.0]").unwrap();
        assert!(d < 0.001);
    }

    #[test]
    fn test_vec_quantize_binary() {
        let result = vec_quantize_binary("[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]");
        assert!(result.is_ok());
    }
}