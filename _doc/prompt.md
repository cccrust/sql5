請在 /Users/Shared/ccc/project/sql5/sql5_pypi/tests/ 下加一系列的 test ，測試 sqlite 的 API 和 sql5 的 API 結果是否一樣，特別是傳回的結構是否相同。

如果不相同，請修改 sql5 (rust + python) ，讓 sql5 的傳回結果，和 sqlite 一致。

CJK 全文檢索的結果格式，應該和 sqlite 一樣，但檢索結果，不需要和 sqlite 一樣

要注意 GROUP 語句的結果，還有 MAX 那些聚合函數，也要一致

記得 rust 和 python 兩端都要做完整詳細的測試，直到完全通過

包含 JOIN, TRANSACTION, ..... 都要一樣