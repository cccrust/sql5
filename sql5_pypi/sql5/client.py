import os
import sys
import json
import subprocess
import tempfile
from typing import Optional, List, Any, Union

class Error(Exception):
    pass

class Cursor:
    def __init__(self, data: dict):
        self.ok = data.get("ok", False)
        self.error = data.get("error")
        self.columns = data.get("columns", [])
        self.rows = data.get("rows", [])
        self.affected = data.get("affected", 0)

    def fetchone(self):
        if self.rows:
            return self.rows[0]
        return None

    def fetchall(self):
        return self.rows

    def __iter__(self):
        return iter(self.rows)

class Connection:
    def __init__(self, path: Optional[str] = None):
        self.path = path
        self._process = None
        self._start_server()

    def _start_server(self):
        binary_path = self._find_binary()
        args = [binary_path, "--server"]
        if self.path:
            args.append(self.path)

        self._process = subprocess.Popen(
            args,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

        line = self._process.stdout.readline()
        if not line:
            stderr = self._process.stderr.read()
            raise Error(f"Server failed to start: {stderr}")

        resp = json.loads(line)
        if not resp.get("ok"):
            raise Error(f"Server init error: {resp}")

    def _find_binary(self) -> str:
        if "SQL5_BINARY" in os.environ:
            return os.environ["SQL5_BINARY"]
        import sql5._binary as _binary
        return _binary.get_binary_path()

    def execute(self, sql: str, params: tuple = ()) -> Cursor:
        sql_with_params = self._substitute_params(sql, params)
        request = {"method": "execute", "sql": sql_with_params}
        self._process.stdin.write(json.dumps(request) + "\n")
        self._process.stdin.flush()

        line = self._process.stdout.readline()
        if not line:
            stderr = self._process.stderr.read()
            raise Error(f"Server error: {stderr}")

        data = json.loads(line)
        return Cursor(data)

    def _substitute_params(self, sql: str, params: tuple) -> str:
        if not params:
            return sql
        result = sql
        for p in params:
            if isinstance(p, int):
                replacement = str(p)
            elif isinstance(p, str):
                replacement = "'" + p.replace("'", "''") + "'"
            elif p is None:
                replacement = "NULL"
            elif isinstance(p, float):
                replacement = str(p)
            else:
                replacement = "'" + str(p).replace("'", "''") + "'"
            result = result.replace("?", replacement, 1)
        return result

    def close(self):
        if self._process:
            try:
                request = {"method": "close"}
                self._process.stdin.write(json.dumps(request) + "\n")
                self._process.stdin.flush()
                self._process.terminate()
                self._process.wait(timeout=5)
            except:
                self._process.kill()
            self._process = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        return False

class WsConnection:
    def __init__(self, path: Optional[str] = None, host: str = "127.0.0.1", port: int = 8080):
        self.path = path
        self.host = host
        self.port = port
        self._process = None
        self._ws = None

        try:
            import websocket
        except ImportError:
            raise Error("websocket-client is required for WebSocket mode: pip install websocket-client")

        self._start_server()
        self._connect_websocket()

    def _start_server(self):
        binary_path = self._find_binary()
        args = [binary_path, "--websocket", str(self.port)]
        if self.path:
            args.append(self.path)

        self._process = subprocess.Popen(
            args,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

    def _find_binary(self) -> str:
        if "SQL5_BINARY" in os.environ:
            return os.environ["SQL5_BINARY"]
        import sql5._binary as _binary
        return _binary.get_binary_path()

    def _connect_websocket(self):
        import websocket
        url = f"ws://{self.host}:{self.port}"
        self._ws = websocket.create_connection(url, timeout=10)

        resp = self._ws.recv()
        data = json.loads(resp)
        if not data.get("ok"):
            raise Error(f"Server init error: {data}")

    def execute(self, sql: str, params: tuple = ()) -> Cursor:
        sql_with_params = self._substitute_params(sql, params)
        request = {"method": "execute", "sql": sql_with_params}
        self._ws.send(json.dumps(request))

        resp = self._ws.recv()
        data = json.loads(resp)
        return Cursor(data)

    def _substitute_params(self, sql: str, params: tuple) -> str:
        if not params:
            return sql
        result = sql
        for p in params:
            if isinstance(p, int):
                replacement = str(p)
            elif isinstance(p, str):
                replacement = "'" + p.replace("'", "''") + "'"
            elif p is None:
                replacement = "NULL"
            elif isinstance(p, float):
                replacement = str(p)
            else:
                replacement = "'" + str(p).replace("'", "''") + "'"
            result = result.replace("?", replacement, 1)
        return result

    def close(self):
        if self._ws:
            try:
                self._ws.send(json.dumps({"method": "close"}))
                self._ws.close()
            except:
                pass
            self._ws = None
        if self._process:
            try:
                self._process.terminate()
                self._process.wait(timeout=5)
            except:
                self._process.kill()
            self._process = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        return False

def connect(
    path: Optional[str] = None,
    transport: str = "subprocess",
    host: str = "127.0.0.1",
    port: int = 8080
) -> Union[Connection, WsConnection]:
    if transport == "websocket":
        return WsConnection(path=path, host=host, port=port)
    return Connection(path)