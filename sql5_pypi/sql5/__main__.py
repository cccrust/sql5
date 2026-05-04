# sql5 命令列入口點
#
# 允許使用 `python -m sql5` 直接執行 Rust 二進位檔。
#
# 使用範例：
#   python -m sql5 --version
#   python -m sql5 mydb.db
#   python -m sql5 --server mydb.db
#   python -m sql5 --websocket 8080 mydb.db

import sys
import os
from sql5._binary import get_binary_path

def main():
    """取得二進位檔路徑並執行，替換當前 Python 程序"""
    binary = get_binary_path()
    os.execv(binary, [binary] + sys.argv[1:])

if __name__ == "__main__":
    main()