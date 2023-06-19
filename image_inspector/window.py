import cv2
import dearpygui.dearpygui as dpg
import numpy as np
from numpy.typing import NDArray


class Window:
    def __init__(self, image: NDArray[np.uint8]):
        self.the_most_original = image.copy()
        self.original_img = image.copy()
        self.width = self.original_img.shape[1]
        self.height = self.original_img.shape[0]
        self.img = image.copy()
        self.properties_id = None
        self.setup_texture()
        self.setup_window()
        self.setup_image_properties()

    @property
    def channels_count(self):
        return self.img.shape[2] if self.img.ndim >= 3 else 1

    @property
    def original_channels_count(self):
        return self.original_img.shape[2] if self.original_img.ndim >= 3 else 1

    def setup_window(self):
        with dpg.window(width=1280, height=720, autosize=True) as window_id:
            with dpg.group(horizontal=True):
                self.setup_plot()
                with dpg.group() as properties:
                    self.properties_id = properties

    def inspect_channel_callback(self, sender, app_data, n):
        img = self.original_img[:, :, n]
        self.original_img = img.copy()
        self.img = img.copy()

        dpg.delete_item(self.properties_id, children_only=True)
        self.setup_image_properties()
        self.render_image()

    def setup_image_properties(self):
        with dpg.group(horizontal=True, parent=self.properties_id):
            dpg.add_button(
                label="Reset original",
                parent=self.properties_id,
                callback=self.reset_original,
            )
            if self.channels_count > 1:
                dpg.add_button(label="BGR to RGB", callback=self.rgb)
                dpg.add_button(label="RGB to HSV", callback=self.rgb_to_hsv)
                dpg.add_button(label="RGB to gray", callback=self.rgb_to_gray)
        with dpg.tab_bar(label="Channels", parent=self.properties_id):
            for n in range(self.channels_count):
                with dpg.tab(label=f"Channel {n}"):
                    upper_thr_id = dpg.generate_uuid()
                    lower_thr_id = dpg.generate_uuid()

                    dpg.add_checkbox(
                        label=f"Enable channel",
                        user_data=(n, lower_thr_id, upper_thr_id),
                        default_value=True,
                        callback=self.channel_checkbox,
                    )
                    dpg.add_slider_int(
                        label="Upper thr",
                        tag=upper_thr_id,
                        user_data=(n, lower_thr_id, upper_thr_id),
                        min_value=0,
                        max_value=255,
                        default_value=255,
                        width=255,
                        callback=self.thr_callback,
                    )

                    dpg.add_slider_int(
                        label="Lower thr",
                        tag=lower_thr_id,
                        user_data=(n, lower_thr_id, upper_thr_id),
                        width=255,
                        min_value=0,
                        max_value=255,
                        default_value=0,
                        callback=self.thr_callback,
                    )
                    if self.channels_count > 1:
                        dpg.add_button(
                            label=f"Inspect separately",
                            callback=self.inspect_channel_callback,
                            user_data=n,
                        )

    def rerender_properties(self):
        dpg.delete_item(self.properties_id, children_only=True)
        self.setup_image_properties()

    def reset_original(self, sender, app_data, user_data):
        self.original_img = self.the_most_original.copy()
        self.img = self.original_img.copy()
        self.rerender_properties()
        self.render_image()

    def setup_plot(self):
        plot_width = 720
        plot_height = round(self.height * (plot_width / self.width))
        if plot_height > 720:
            plot_height = 720
            plot_width = round(self.width * (plot_height / self.height))
        with dpg.plot(
            width=plot_width,
            height=plot_height,
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
                    self.texture,
                    bounds_min=(0, self.height),
                    bounds_max=(self.width, 0),  # to avoid image flip
                )
                with dpg.item_handler_registry() as handler_id:
                    dpg.add_item_hover_handler(callback=print)

    def setup_texture(self):
        with dpg.texture_registry(show=False) as texture_registry_id:
            self.texture_registry_id = texture_registry_id

            self.texture = dpg.add_dynamic_texture(
                width=self.width, height=self.height, default_value=[]
            )
        self.render_image()

    def render_image(self):
        dpg.set_value(self.texture, self.dpg_img)

    @property
    def dpg_img(self):
        match self.img.ndim:
            case 2:
                img = cv2.cvtColor(self.img, cv2.COLOR_GRAY2RGBA)
            case 3:
                match self.img.shape[2]:
                    case 1:
                        raise ValueError("Unexpected image with 1 channels")
                    case 2:
                        raise ValueError("Unexpected image with 2 channels")
                    case 3:
                        img = np.dstack(
                            (
                                self.img,
                                np.full(self.img.shape[:2], fill_value=255, dtype=np.uint8),
                            )
                        )
                        pass
                    case 4:
                        img = self.img
                    case channels_count:
                        raise ValueError(f"Unexpected image with {channels_count} channels")
            case ndim:
                raise ValueError(f"Unexpected image with {ndim} dimensions")

        img = np.array(img, dtype=np.float32).ravel() / 255
        return img

    def thr_callback(self, sender, thr, user_data):
        n, lower_thr_id, upper_thr_id = user_data
        if sender == lower_thr_id:
            lower_thr = thr
            upper_thr = dpg.get_value(upper_thr_id)
        elif sender == upper_thr_id:
            upper_thr = thr
            lower_thr = dpg.get_value(lower_thr_id)
        else:
            raise NotImplementedError(f"Unexpected sender {sender} for 'thr_callback'")

        if self.channels_count == 1:
            channel = self.original_img
            self.img = np.where((channel < upper_thr) & (channel > lower_thr), channel, 0)
        else:
            channel = self.original_img[:, :, n]
            self.img[:, :, n] = np.where((channel < upper_thr) & (channel > lower_thr), channel, 0)

        self.render_image()

    def channel_checkbox(self, sender, flag, user_data):
        n, lower_thr_id, upper_thr_id = user_data
        new_value = 255 if flag else 0
        dpg.set_value(upper_thr_id, new_value)
        dpg.set_value(lower_thr_id, 0)
        self.thr_callback(upper_thr_id, new_value, (n, lower_thr_id, upper_thr_id))

    def rgb(self):
        if self.channels_count != 3:
            raise ValueError(f"Expected 3 channels, got {self.channels_count}")
        self.original_img = cv2.cvtColor(self.original_img, cv2.COLOR_BGR2RGB)
        self.img = self.original_img.copy()
        self.rerender_properties()
        self.render_image()

    def remove_alpha(self):
        if self.channels_count != 4:
            raise ValueError(f"Expected 4 channels, got {self.channels_count}")
        self.original_img = self.original_img[:, :, :3].copy()
        self.img = self.original_img.copy()
        self.rerender_properties()
        self.render_image()

    def rgb_to_hsv(self):
        self.original_img = cv2.cvtColor(self.original_img, cv2.COLOR_RGB2HSV)
        self.img = self.original_img.copy()
        self.rerender_properties()
        self.render_image()

    def rgb_to_gray(self):
        self.original_img = cv2.cvtColor(self.original_img, cv2.COLOR_RGB2GRAY)
        self.img = self.original_img.copy()
        self.rerender_properties()
        self.render_image()
