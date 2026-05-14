//! 向量資料類型與操作
//!
//! 支援三種向量類型：
//! - float32: 32 位元浮點數向量
//! - int8: 8 位元整數向量
//! - bit: 二值向量

use serde::{Deserialize, Serialize};

/// 向量類型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorType {
    Float32(Vec<f32>),
    Int8(Vec<i8>),
    Bit(Vec<u8>),
}

impl VectorType {
    /// 取得向量維度
    pub fn dimension(&self) -> usize {
        match self {
            VectorType::Float32(v) => v.len(),
            VectorType::Int8(v) => v.len(),
            VectorType::Bit(v) => v.len() * 8,
        }
    }

    /// 取得類型名稱
    pub fn type_name(&self) -> &'static str {
        match self {
            VectorType::Float32(_) => "float32",
            VectorType::Int8(_) => "int8",
            VectorType::Bit(_) => "bit",
        }
    }

    /// 從 JSON 字串解析向量
    pub fn from_json(json: &str, type_name: &str) -> Result<Self, String> {
        let json = json.trim();
        if !json.starts_with('[') {
            return Err("JSON must be an array".to_string());
        }

        match type_name.to_lowercase().as_str() {
            "float32" | "float" | "f32" => {
                let values: Vec<f32> = serde_json::from_str(json)
                    .map_err(|e| format!("JSON parsing error: {}", e))?;
                Ok(VectorType::Float32(values))
            }
            "int8" | "int" | "i8" => {
                let values: Vec<i32> = serde_json::from_str(json)
                    .map_err(|e| format!("JSON parsing error: {}", e))?;
                let int8_values: Vec<i8> = values.iter()
                    .map(|&v| {
                        if v < -128 || v > 127 {
                            panic!("JSON parsing error: value out of range for int8");
                        }
                        v as i8
                    })
                    .collect();
                Ok(VectorType::Int8(int8_values))
            }
            "bit" | "binary" => {
                let values: Vec<i32> = serde_json::from_str(json)
                    .map_err(|e| format!("JSON parsing error: {}", e))?;
                let mut bytes = Vec::new();
                for chunk in values.chunks(8) {
                    let mut byte = 0u8;
                    for (i, &v) in chunk.iter().enumerate() {
                        if v != 0 {
                            byte |= 1 << i;
                        }
                    }
                    bytes.push(byte);
                }
                Ok(VectorType::Bit(bytes))
            }
            _ => Err(format!("Unknown vector type: {}", type_name)),
        }
    }

    /// 從 BLOB 解析向量
    pub fn from_blob(blob: &[u8], type_name: &str) -> Result<Self, String> {
        match type_name.to_lowercase().as_str() {
            "float32" | "float" | "f32" => {
                if blob.len() % 4 != 0 {
                    return Err(format!(
                        "invalid float32 vector BLOB length. Must be divisible by 4, found {}",
                        blob.len()
                    ));
                }
                let mut values = Vec::with_capacity(blob.len() / 4);
                for chunk in blob.chunks(4) {
                    let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    values.push(value);
                }
                Ok(VectorType::Float32(values))
            }
            "int8" | "int" | "i8" => {
                Ok(VectorType::Int8(blob.iter().map(|&b| b as i8).collect()))
            }
            "bit" | "binary" => {
                Ok(VectorType::Bit(blob.to_vec()))
            }
            _ => Err(format!("Unknown vector type: {}", type_name)),
        }
    }

    /// 轉為 JSON 字串
    pub fn to_json(&self) -> String {
        match self {
            VectorType::Float32(v) => {
                let vals: Vec<String> = v.iter().map(|f| format!("{}.000000", f)).collect();
                format!("[{}]", vals.join(","))
            }
            VectorType::Int8(v) => {
                let vals: Vec<String> = v.iter().map(|i| i.to_string()).collect();
                format!("[{}]", vals.join(","))
            }
            VectorType::Bit(v) => {
                let mut vals = Vec::new();
                for byte in v {
                    for i in 0..8 {
                        vals.push(if (byte & (1 << i)) != 0 { "1" } else { "0" }.to_string());
                    }
                }
                format!("[{}]", vals.join(","))
            }
        }
    }

    /// 轉為 BLOB
    pub fn to_blob(&self) -> Vec<u8> {
        match self {
            VectorType::Float32(v) => {
                let mut blob = Vec::with_capacity(v.len() * 4);
                for f in v {
                    blob.extend_from_slice(&f.to_le_bytes());
                }
                blob
            }
            VectorType::Int8(v) => v.iter().map(|&b| b as u8).collect(),
            VectorType::Bit(v) => v.clone(),
        }
    }

    /// 向量相加 (僅 float32 和 int8)
    pub fn add(&self, other: &VectorType) -> Result<Self, String> {
        match (self, other) {
            (VectorType::Float32(a), VectorType::Float32(b)) => {
                if a.len() != b.len() {
                    return Err("Vector length mismatch".to_string());
                }
                let result: Vec<f32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Ok(VectorType::Float32(result))
            }
            (VectorType::Int8(a), VectorType::Int8(b)) => {
                if a.len() != b.len() {
                    return Err("Vector length mismatch".to_string());
                }
                let result: Vec<i8> = a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_add(*y)).collect();
                Ok(VectorType::Int8(result))
            }
            _ => Err("Cannot add vectors of different types".to_string()),
        }
    }

    /// 向量相減 (僅 float32 和 int8)
    pub fn sub(&self, other: &VectorType) -> Result<Self, String> {
        match (self, other) {
            (VectorType::Float32(a), VectorType::Float32(b)) => {
                if a.len() != b.len() {
                    return Err("Vector length mismatch".to_string());
                }
                let result: Vec<f32> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
                Ok(VectorType::Float32(result))
            }
            (VectorType::Int8(a), VectorType::Int8(b)) => {
                if a.len() != b.len() {
                    return Err("Vector length mismatch".to_string());
                }
                let result: Vec<i8> = a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_sub(*y)).collect();
                Ok(VectorType::Int8(result))
            }
            _ => Err("Cannot subtract vectors of different types".to_string()),
        }
    }

    /// 向量 L2 正規化 (僅 float32)
    pub fn normalize(&self) -> Result<Self, String> {
        match self {
            VectorType::Float32(v) => {
                let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
                if norm == 0.0 {
                    return Err("Cannot normalize zero vector".to_string());
                }
                let result: Vec<f32> = v.iter().map(|x| x / norm).collect();
                Ok(VectorType::Float32(result))
            }
            _ => Err("Only float32 vectors can be normalized".to_string()),
        }
    }

    /// 擷取子向量
    pub fn slice(&self, start: usize, end: usize) -> Result<Self, String> {
        if start >= end {
            return Err("start must be less than end".to_string());
        }
        if end > self.dimension() {
            return Err("end index exceeds vector dimension".to_string());
        }

        match self {
            VectorType::Float32(v) => {
                if start >= v.len() || end > v.len() {
                    return Err("slice indices out of bounds".to_string());
                }
                Ok(VectorType::Float32(v[start..end].to_vec()))
            }
            VectorType::Int8(v) => {
                if start >= v.len() || end > v.len() {
                    return Err("slice indices out of bounds".to_string());
                }
                Ok(VectorType::Int8(v[start..end].to_vec()))
            }
            VectorType::Bit(v) => {
                if start % 8 != 0 || end % 8 != 0 {
                    return Err("bit vector slice indices must be multiples of 8".to_string());
                }
                let start_byte = start / 8;
                let end_byte = end / 8;
                Ok(VectorType::Bit(v[start_byte..end_byte].to_vec()))
            }
        }
    }

    /// 二值化量化
    pub fn quantize_binary(&self) -> Result<Self, String> {
        match self {
            VectorType::Float32(v) => {
                if v.len() % 8 != 0 {
                    return Err("Binary quantization requires vectors with a length divisible by 8".to_string());
                }
                let mut bytes = Vec::new();
                for chunk in v.chunks(8) {
                    let mut byte = 0u8;
                    for (i, &f) in chunk.iter().enumerate() {
                        if f >= 0.0 {
                            byte |= 1 << i;
                        }
                    }
                    bytes.push(byte);
                }
                Ok(VectorType::Bit(bytes))
            }
            VectorType::Int8(v) => {
                if v.len() % 8 != 0 {
                    return Err("Binary quantization requires vectors with a length divisible by 8".to_string());
                }
                let mut bytes = Vec::new();
                for chunk in v.chunks(8) {
                    let mut byte = 0u8;
                    for (i, &f) in chunk.iter().enumerate() {
                        if f >= 0 {
                            byte |= 1 << i;
                        }
                    }
                    bytes.push(byte);
                }
                Ok(VectorType::Bit(bytes))
            }
            VectorType::Bit(_) => Err("Can only binary quantize float or int8 vectors".to_string()),
        }
    }
}

/// 解析維度標記，例如 "float[768]" -> (768, "float")
pub fn parse_dimension_type(type_str: &str) -> Result<(usize, &str), String> {
    let type_str = type_str.trim();

    // 檢查格式：type[dimension] 或 type
    if let Some(bracket_pos) = type_str.find('[') {
        if !type_str.ends_with(']') {
            return Err("Invalid dimension format".to_string());
        }

        let base_type = &type_str[..bracket_pos];
        let dim_str = &type_str[bracket_pos+1..type_str.len()-1];
        let dimension: usize = dim_str.parse()
            .map_err(|_| "Invalid dimension number".to_string())?;

        Ok((dimension, base_type))
    } else {
        Err("Dimension must be specified, e.g., float[768]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float32_from_json() {
        let v = VectorType::from_json("[1.0, 2.0, 3.0]", "float32").unwrap();
        assert!(matches!(v, VectorType::Float32(_)));
        if let VectorType::Float32(f) = v {
            assert_eq!(f, vec![1.0, 2.0, 3.0]);
        }
    }

    #[test]
    fn test_int8_from_json() {
        let v = VectorType::from_json("[1, 2, 3, 4]", "int8").unwrap();
        assert!(matches!(v, VectorType::Int8(_)));
    }

    #[test]
    fn test_to_json() {
        let v = VectorType::Float32(vec![1.0, 2.0, 3.0]);
        let json = v.to_json();
        assert!(json.contains("1.000000"));
    }

    #[test]
    fn test_normalize() {
        let v = VectorType::Float32(vec![2.0, 0.0, 0.0]);
        let normalized = v.normalize().unwrap();
        if let VectorType::Float32(f) = normalized {
            assert!((f[0] - 1.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_add() {
        let a = VectorType::Float32(vec![1.0, 2.0]);
        let b = VectorType::Float32(vec![3.0, 4.0]);
        let c = a.add(&b).unwrap();
        if let VectorType::Float32(f) = c {
            assert_eq!(f, vec![4.0, 6.0]);
        }
    }

    #[test]
    fn test_quantize_binary() {
        let v = VectorType::Float32(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let quantized = v.quantize_binary().unwrap();
        assert!(matches!(quantized, VectorType::Bit(_)));
    }

    #[test]
    fn test_parse_dimension() {
        assert_eq!(parse_dimension_type("float[768]").unwrap(), (768, "float"));
        assert_eq!(parse_dimension_type("int8[128]").unwrap(), (128, "int8"));
    }
}