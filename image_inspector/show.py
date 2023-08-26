import numpy as np
from numpy.typing import NDArray

from image_inspector.inspector import ImageInspector


def show(*images: NDArray[np.uint8]):


    ImageInspector(*images)
