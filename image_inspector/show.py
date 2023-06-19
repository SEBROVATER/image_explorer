from typing import Iterable

import dearpygui.dearpygui as dpg
import numpy as np
from numpy.typing import NDArray

from image_inspector.window import Window


def validate_input(images: Iterable[NDArray[np.uint8]]):
    for i, img in enumerate(images, 1):
        assert 2 <= img.ndim <= 3, f"Images must have 2 or 3 dimensions. Got {img.ndim}"
        if img.ndim == 3:
            assert img.shape[2] in {
                1,
                3,
                4,
            }, f"Images must have 1 or 3 or 4 channels. Got {img.shape[2]} for {i}-nth"


def show(images: Iterable[NDArray[np.uint8]]):
    images = tuple(images)
    validate_input(images)

    dpg.create_context()
    dpg.create_viewport(
        title="Image Explorer",
    )
    for img in images:
        Window(img)

    dpg.setup_dearpygui()

    dpg.show_viewport(maximized=True)
    dpg.start_dearpygui()
    dpg.destroy_context()
