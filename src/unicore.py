from sys import argv, exit

from envs import variables as var
from envs import error_handler as err
from modules import version

def usage(_):
    print(f"Unicore v{var.VERSION} {var.STABLE_TEXT}")
    print("Usage: unicore <module> [args]")
    print("")
    print("Available modules:")
    print("  version     - Print version and information")
    print("  more to come...")
    print("")

VALID_MODULES = {
    "help": usage,
    "usage": usage,
    "info": version.run,
    "version": version.run
}
def init():
    var.config_init(f'{var.PARENT_DIR}/path.cfg')
    if len(argv) < 2:
        usage(None)
        exit(0)
    if not argv[1] in VALID_MODULES:
        err.warning(err.ERR_MODULE_NOT_FOUND, argv[1])
        usage(None)
        exit(1)



def run():
    VALID_MODULES[argv[1]](argv[2:])

def main():
    init()
    run()

if __name__ == "__main__":
    main()