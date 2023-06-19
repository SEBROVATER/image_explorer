import os
import signal
from functools import cache
from pathlib import Path


@cache
def get_base_dir():
    base_dir = Path(__file__).parent
    while "utils" not in (p.name for p in base_dir.iterdir()):
        base_dir = base_dir.parent

    return base_dir


@cache
def get_data_dir():
    return get_base_dir().parent / "data" / "redhorse"


@cache
def is_in_debug():
    """Returns False if there is a DEBUG environment variable with
    value 'False' or 'false', in any other cases returns True"""

    return os.environ.get("DEBUG", "True").lower() != "false"


def load_envs(name: str):
    from dotenv import load_dotenv

    assert name.endswith(".env"), ".env file must end with '.env'"
    load_dotenv(get_base_dir() / name, override=True)


@cache
def get_main_screen_size() -> tuple[int, int]:
    from mss import mss

    with mss() as sct:
        for monitor in reversed(sct.monitors):
            if monitor["left"] == 0 and monitor["top"] == 0:
                return monitor["width"], monitor["height"]


class GracefulKiller:
    is_running = True

    def __init__(self):
        signal.signal(signal.SIGINT, self.exit_gracefully)
        signal.signal(signal.SIGTERM, self.exit_gracefully)

    def exit_gracefully(self, *args):
        GracefulKiller.is_running = False
