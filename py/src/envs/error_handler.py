# Basic error handling for the environment
# Recieves error code and message, and prints the message to stderr
import sys

ERR_UNKNOWN = 0x01
ERR_FILE_NOT_FOUND = 0x02
ERR_MODULE_NOT_FOUND = 0x03
def build_message(code: int, passed_object: str = '') -> str:
    if code == ERR_UNKNOWN:
        return f"Unknown error"
    elif code == ERR_FILE_NOT_FOUND:
        return f"File not found: {passed_object}"
    elif code == ERR_MODULE_NOT_FOUND:
        return f"Module not found: {passed_object}"
    else:
        return f"Unrecognized error code: {code}"

# warning: prints message to stderr but does not exit
def warning(code: int, passed_object: str = ''):
    print(build_message(code, passed_object), file=sys.stderr)

# error: prints message to stderr and exits
def error(code: int, passed_object: str = ''):
    warning(code, passed_object)
    sys.exit(code)