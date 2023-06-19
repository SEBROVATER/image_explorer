from typing import Iterable

import dearpygui.dearpygui as dpg
import numpy as np
from numpy.typing import NDArray

from image_explorer.window import Window


def run(images: Iterable[NDArray[np.uint8]]):
    dpg.create_context()

    # dpg.show_metrics()
    # dpg.show_debug()
    # dpg.show_about()
    # dpg.show_item_registry()
    dpg.create_viewport(
        title="Image Explorer",
        # width=1920, height=1080,
    )
    for img in images:
        Window(img)

    dpg.setup_dearpygui()

    dpg.show_viewport(maximized=False)
    dpg.start_dearpygui()
    dpg.destroy_context()
