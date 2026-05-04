import sys
import os
from sql5._binary import get_binary_path

def main():
    binary = get_binary_path()
    os.execv(binary, [binary] + sys.argv[1:])

if __name__ == "__main__":
    main()