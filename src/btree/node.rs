/// B+Tree 節點類型
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    /// 內部節點：存放 key 與子節點指標
    Internal,
    /// 葉節點：存放實際資料，並以鏈結串列相連
    Leaf,
}

/// 通用 key 類型（支援整數與字串）
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    Integer(i64),
    Text(String),
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Integer(v) => write!(f, "{}", v),
            Key::Text(s) => write!(f, "{}", s),
        }
    }
}

/// 資料列（Row）：以 key-value bytes 儲存
#[derive(Debug, Clone)]
pub struct Record {
    pub key: Key,
    pub value: Vec<u8>,
}

/// B+Tree 節點
#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    /// 節點中的 key 列表
    pub keys: Vec<Key>,
    /// 僅內部節點使用：子節點索引（長度 = keys.len() + 1）
    pub children: Vec<usize>,
    /// 僅葉節點使用：對應 key 的 record 資料
    pub records: Vec<Record>,
    /// 葉節點鏈結：指向下一個葉節點的索引（None 表示最後一個）
    pub next_leaf: Option<usize>,
}

impl Node {
    /// 建立空白內部節點
    pub fn new_internal() -> Self {
        Node {
            node_type: NodeType::Internal,
            keys: Vec::new(),
            children: Vec::new(),
            records: Vec::new(),
            next_leaf: None,
        }
    }

    /// 建立空白葉節點
    pub fn new_leaf() -> Self {
        Node {
            node_type: NodeType::Leaf,
            keys: Vec::new(),
            children: Vec::new(),
            records: Vec::new(),
            next_leaf: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }

    pub fn is_full(&self, order: usize) -> bool {
        self.keys.len() >= order - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_internal() {
        let nt = NodeType::Internal;
        assert!(matches!(nt, NodeType::Internal));
    }

    #[test]
    fn test_node_type_leaf() {
        let nt = NodeType::Leaf;
        assert!(matches!(nt, NodeType::Leaf));
    }

    #[test]
    fn test_key_integer() {
        let k = Key::Integer(42);
        assert_eq!(k.to_string(), "42");
        assert_eq!(k, Key::Integer(42));
        assert_ne!(k, Key::Integer(43));
    }

    #[test]
    fn test_key_text() {
        let k = Key::Text("hello".to_string());
        assert_eq!(k.to_string(), "hello");
        assert_eq!(k, Key::Text("hello".to_string()));
        assert_ne!(k, Key::Text("world".to_string()));
    }

    #[test]
    fn test_key_ordering() {
        let k1 = Key::Integer(1);
        let k2 = Key::Integer(2);
        assert!(k1 < k2);

        let t1 = Key::Text("a".to_string());
        let t2 = Key::Text("b".to_string());
        assert!(t1 < t2);
    }

    #[test]
    fn test_key_clone() {
        let k = Key::Integer(100);
        let cloned = k.clone();
        assert_eq!(k, cloned);
    }

    #[test]
    fn test_record() {
        let record = Record {
            key: Key::Integer(1),
            value: vec![1, 2, 3],
        };
        assert_eq!(record.key, Key::Integer(1));
        assert_eq!(record.value, vec![1, 2, 3]);
    }

    #[test]
    fn test_record_clone() {
        let record = Record {
            key: Key::Text("test".to_string()),
            value: vec![10, 20],
        };
        let cloned = record.clone();
        assert_eq!(record.key, cloned.key);
        assert_eq!(record.value, cloned.value);
    }

    #[test]
    fn test_node_new_internal() {
        let node = Node::new_internal();
        assert!(matches!(node.node_type, NodeType::Internal));
        assert!(node.keys.is_empty());
        assert!(node.children.is_empty());
        assert!(node.records.is_empty());
        assert!(node.next_leaf.is_none());
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_node_new_leaf() {
        let node = Node::new_leaf();
        assert!(matches!(node.node_type, NodeType::Leaf));
        assert!(node.keys.is_empty());
        assert!(node.children.is_empty());
        assert!(node.records.is_empty());
        assert!(node.next_leaf.is_none());
        assert!(node.is_leaf());
    }

    #[test]
    fn test_node_is_full() {
        let mut node = Node::new_internal();
        assert!(!node.is_full(4));

        for i in 0..3 {
            node.keys.push(Key::Integer(i));
        }
        assert!(node.is_full(4));

        node.keys.push(Key::Integer(3));
        assert!(node.is_full(4));
    }

    #[test]
    fn test_node_is_full_leaf() {
        let mut node = Node::new_leaf();
        for i in 0..2 {
            node.keys.push(Key::Integer(i));
        }
        assert!(node.is_full(3));
    }

    #[test]
    fn test_node_clone() {
        let mut node = Node::new_internal();
        node.keys.push(Key::Integer(1));
        node.children.push(10);

        let cloned = node.clone();
        assert_eq!(node.keys, cloned.keys);
        assert_eq!(node.children, cloned.children);
    }

    #[test]
    fn test_node_with_keys() {
        let mut node = Node::new_leaf();
        node.keys.push(Key::Integer(1));
        node.keys.push(Key::Integer(2));
        node.keys.push(Key::Text("hello".to_string()));

        assert_eq!(node.keys.len(), 3);
        assert!(!node.is_leaf() || node.keys.len() == 3);
    }

    #[test]
    fn test_node_next_leaf() {
        let mut node = Node::new_leaf();
        assert!(node.next_leaf.is_none());

        node.next_leaf = Some(100);
        assert_eq!(node.next_leaf, Some(100));
    }

    #[test]
    fn test_node_with_records() {
        let mut node = Node::new_leaf();
        node.records.push(Record {
            key: Key::Integer(1),
            value: vec![1, 2],
        });
        node.records.push(Record {
            key: Key::Integer(2),
            value: vec![3, 4],
        });

        assert_eq!(node.records.len(), 2);
    }
}
