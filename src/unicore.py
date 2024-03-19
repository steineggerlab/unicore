import envs.variables as var

def main():
    var.config_init(f'{var.PARENT_DIR}/path.cfg')
    print(f'Unicore v{var.VERSION}')

if __name__ == "__main__":
    main()