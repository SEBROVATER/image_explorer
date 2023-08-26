from cProfile import label
from typing import Iterable

import cv2
import dearpygui.dearpygui as dpg
import numpy as np
from numpy.typing import NDArray

from image_inspector.child_window import ChildWindow


class ImageInspector:
    def __init__(self, *images: NDArray[np.uint8]):
        images = self.validate_input(images)

        dpg.create_context()
        dpg.create_viewport(
            title="Image Explorer",
        )
        # dpg.show_style_editor()
        # dpg.show_item_registry()
        with dpg.window() as primary_window:
            dpg.set_primary_window(primary_window, value=True)
            self.windows = dict()

            with dpg.group(horizontal=True, horizontal_spacing=50) as image_selector:
                ...



            with dpg.group(horizontal=True):

                for i, img in enumerate(images):
                    win = ChildWindow(img, i, len(images))
                    self.windows[win.id] = win
                    dpg.add_checkbox(
                        label=str(i),
                        user_data=win.id,
                        parent=image_selector,
                        default_value=i == 0,
                        callback=self.show_subinpector_callback)

        dpg.maximize_viewport()
        dpg.setup_dearpygui()
        dpg.show_viewport(maximized=True)

        dpg.start_dearpygui()
        dpg.destroy_context()

    def show_subinpector_callback(self, sender, app_data, user_data):
        if app_data:
            dpg.show_item(user_data)
        else:
            dpg.hide_item(user_data)


    @staticmethod
    def validate_input(images: Iterable[NDArray[np.uint8]]) -> tuple[NDArray[np.uint8], ...]:
        images = tuple(images)
        for i, img in enumerate(images, 1):
            assert isinstance(img, np.ndarray), f"Images must be numpy arrays, got {type(img)}"
            assert img.dtype == np.uint8, f"Images must be type of uint8, got {img.dtype}"
            assert 2 <= img.ndim <= 3, f"Images must have 2 or 3 dimensions, got {img.ndim}"
            if img.ndim == 3:
                assert img.shape[2] <= 4, f"Images must have up to 4 channels, got {img.shape[2]}"
        return images

if __name__ == "__main__":
    ImageInspector(cv2.imread("../cropped1.png"), cv2.imread("../cropped1.png"))