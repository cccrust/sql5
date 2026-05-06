# sql5 Python 客戶端套件
#
# 提供與 DB-API 2.0 相容的介面，用於連接 sql5 資料庫。
#
# 使用範例：
#   import sql5
#   conn = sql5.connect("mydb.db")
#   cursor = conn.execute("SELECT * FROM users")
#   for row in cursor:
#       print(row)
#   conn.close()

__version__ = "3.2.5"
__all__ = ["connect", "Connection", "Cursor", "Error"]

from .client import connect, Connection, Cursor, Error