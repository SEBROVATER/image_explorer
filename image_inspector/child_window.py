from typing import Optional

import cv2
import dearpygui.dearpygui as dpg
import numpy as np
from numpy.typing import NDArray


class ChildWindow:
    def __init__(self, image: NDArray[np.uint8], win_idx: int, win_count: int):
        self.original_img = image.copy()
        self.win_idx = win_idx
        self.win_count = win_count
        self.main_img = image.copy()

        self.channels_img = cv2.split(self.original_img)

        empty_image = np.empty((self.img_h, self.img_w, 4), dtype=np.uint8).ravel().astype(np.float64) / 255
        with dpg.texture_registry(show=False):
            self.main_texture = dpg.add_dynamic_texture(
                width=self.img_w, height=self.img_h, default_value=empty_image
            )
            self.channels_textures = tuple(
                dpg.add_dynamic_texture(
                    width=self.img_w, height=self.img_h, default_value=empty_image
                )
                for _ in range(self.channels_count)
            )
        self.id = self.setup_window()
        self.render_image(0)

    @property
    def img_w(self):
        return self.original_img.shape[1]

    @property
    def img_h(self):
        return self.original_img.shape[0]

    @property
    def channels_count(self):
        return self.original_img.shape[2] if self.original_img.ndim == 3 else 1

    def setup_window(self):
        with dpg.child_window(width=720, show=self.win_idx == 0) as window_id:
            dpg.add_text(default_value=f"{self.win_idx}")
            self.setup_plots()
        return window_id





    def setup_plots(self):
        with dpg.group(horizontal=True):
            with dpg.plot(
                width=640,
                height=round(self.img_h * 640 / self.img_w),
                query=False,
                no_title=True,
                # no_menus=True,
                no_box_select=True,
                crosshairs=True,
                pan_button=dpg.mvMouseButton_Middle,
                # query_button=dpg.mvMouseButton_Left,
            ) as plot_id:
                x_axis = dpg.add_plot_axis(dpg.mvXAxis)
                with dpg.plot_axis(dpg.mvYAxis, invert=True) as y_axis:
                    dpg.add_image_series(
                        self.main_texture,
                        bounds_min=(0, self.img_h),
                        bounds_max=(self.img_w, 0),  # to avoid image flip
                    )
        for i in range(self.channels_count):
            dpg.add_text(default_value=f"Channel {i}")
            with dpg.group(horizontal=True):
                with dpg.plot(
                    width=600,
                    height=round(self.img_h * 600 / self.img_w),
                    query=False,
                    no_title=True,
                    # no_menus=True,
                    no_box_select=True,
                    crosshairs=True,
                    pan_button=dpg.mvMouseButton_Middle,
                    # query_button=dpg.mvMouseButton_Left,
                ) as plot_id:
                    x_axis = dpg.add_plot_axis(dpg.mvXAxis)
                    with dpg.plot_axis(dpg.mvYAxis, invert=True) as y_axis:
                        dpg.add_image_series(
                            self.channels_textures[i],
                            bounds_min=(0, self.img_h),
                            bounds_max=(self.img_w, 0),  # to avoid image flip
                        )
                dpg.add_slider_int(label="[", clamped=True,
                    tag=f"min_slider_{self.win_idx}_{i}",user_data=i,default_value=0,height=round(self.img_h * 600 / self.img_w),
                                   min_value=0, max_value=255, vertical=True, callback=self.slider_callback
                )
                dpg.add_slider_int(label=")", clamped=True,
                    tag=f"max_slider_{self.win_idx}_{i}", default_value=256, user_data=i, height=round(self.img_h * 600 / self.img_w),
                                   min_value=1, max_value=256, vertical=True, callback=self.slider_callback
                                   )

    def slider_callback(self, sender: str, app_data: int, user_data: int):
        # TODO: add i and slider ids to user_data


        if sender.startswith("min"):
            dpg.configure_item(f"max_slider_{self.win_idx}_{user_data}", min_value=app_data)
        elif sender.startswith("max"):
            dpg.configure_item(f"min_slider_{self.win_idx}_{user_data}", max_value=app_data)

        self.render_image(with_channel=user_data)


    def render_image(self, with_channel: Optional[int] = None):
        channels_imgs = self.get_processed_channels()

        if with_channel is not None:
            img = channels_imgs[with_channel]
            img = cv2.cvtColor(img, cv2.COLOR_GRAY2RGBA)
            img = self.cast_to_dpg(img)
            texture_id = self.channels_textures[with_channel]
            dpg.set_value(texture_id, img)

        match len(channels_imgs):
            case 1:
                img = np.dstack(channels_imgs)
                img = cv2.cvtColor(img, cv2.COLOR_GRAY2RGBA)
            case 3:
                alpha = np.full((self.img_h, self.img_w), fill_value=255, dtype=np.uint8)
                img = np.dstack((*channels_imgs, alpha))
            case 4:
                img = np.dstack(channels_imgs)
            case channels_count:
                raise ValueError(f"Unexpected image with {channels_count} channels")

        img = self.cast_to_dpg(img)
        dpg.set_value(self.main_texture, img)

    @staticmethod
    def cast_to_dpg(rgba):
        return (rgba.ravel() / 255).astype(np.float64)


    def get_processed_channels(self):
        channels_img = list()
        for i, img in enumerate(self.channels_img):
            lower_thr = dpg.get_value(f"min_slider_{self.win_idx}_{i}")
            upper_thr = dpg.get_value(f"max_slider_{self.win_idx}_{i}")

            img = np.where((img < upper_thr) & (img >= lower_thr), img, 0)
            channels_img.append(img)
        return channels_img

