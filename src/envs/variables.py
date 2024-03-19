# Global variables and constants
import os
from pathlib import Path
import src.envs.error_handler as err

VERSION = "0.0.1"
STABLE = False
STABLE_TEXT = "Stable" if STABLE else "Unstable"

# Enviornmental paths
WORKING_DIR = Path(os.getcwd())
PARENT_DIR = WORKING_DIR.parent.absolute()

# Binary paths
# Use default -> config file -> arguments
BIN_MMSEQS = "mmseqs"
def set_mmseqs(path: str):
    global BIN_MMSEQS
    BIN_MMSEQS = path
def test_mmseqs() -> bool: pass

BIN_FOLDSEEK = "foldseek"
def set_foldseek(path: str):
    global BIN_FOLDSEEK
    BIN_FOLDSEEK = path
def test_foldseek() -> bool: pass

BIN_MAFFT = "mafft"
def set_mafft(path: str):
    global BIN_MAFFT
    BIN_MAFFT = path
def test_mafft() -> bool: pass

BIN_MAFFT_LINSI = "mafft-linsi"
def set_mafft_linsi(path: str):
    global BIN_MAFFT_LINSI
    BIN_MAFFT_LINSI = path
def test_mafft_linsi() -> bool: pass

BIN_IQTREE = "iqtree"
def set_iqtree(path: str):
    global BIN_IQTREE
    BIN_IQTREE = path
def test_iqtree() -> bool: pass

BIN_FASTTREE = "fasttree"
def set_fasttree(path: str):
    global BIN_FASTTREE
    BIN_FASTTREE = path
def test_fasttree() -> bool: pass

# read config file and update binary paths
VALID_KEYMAP = {
    "mmseqs": set_mmseqs,
    "foldseek": set_foldseek,
    "mafft": set_mafft,
    "mafft-linsi": set_mafft_linsi,
    "iqtree": set_iqtree,
    "fasttree": set_fasttree
}
def config_init(config_file: str):
    if not os.path.exists(config_file):
        err.error(err.ERR_FILE_NOT_FOUND, config_file)
    with open(config_file, 'r') as f:
        while buf := f.readline().rstrip():
            if buf.startswith("#"): continue
            key, value = buf.split("=")
            if key in VALID_KEYMAP:
                VALID_KEYMAP[key](value)

