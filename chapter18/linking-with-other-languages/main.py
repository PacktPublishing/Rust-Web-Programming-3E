# main.py
import ctypes
import platform
system = platform.system()

if system == "Linux":
    lib = ctypes.CDLL("./target/release/libto_do_c.so")
elif system == "Darwin":
    lib = ctypes.CDLL("./target/release/libto_do_c.dylib")
elif system == "Windows":
    lib = ctypes.CDLL("./target/release/to_do_c.dll")
else:
    raise OSError("Unsupported platform: " + system)

# Define the argument and return types for the Rust functions
lib.create_todo_json.argtypes = [ctypes.c_char_p]
lib.create_todo_json.restype = ctypes.c_char_p
lib.mark_done_json.argtypes = [ctypes.c_char_p]
lib.mark_done_json.restype = ctypes.c_char_p


def create_todo_json(title):
    todo_json = lib.create_todo_json(title.encode("utf-8"))
    todo_py = todo_json.decode("utf-8")
    return todo_py


def mark_done_json(todo_json):
    updated_todo_json = lib.mark_done_json(todo_json.encode("utf-8"))
    updated_todo_py = updated_todo_json.decode("utf-8")
    return updated_todo_py


if __name__ == "__main__":
    todo_json = create_todo_json("Learn Rust")
    print(f"Todo JSON: {todo_json}")
    updated_todo_json = mark_done_json(todo_json)
    print(f"Updated Todo JSON: {updated_todo_json}")
