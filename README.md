# imspect
A small GUI application for feature engineering in computer vision projects.

- Very useful in a **debugger** session.
- Non-blocking. Executes in an independent process.
- Lightweight, has zero dependencies.
- Works only with `numpy` images with data type `uint8`
 (common data type for `OpenCV`).
- The command line executable accepts the classic image formats
  and additionally the **.npy** format (serialized `numpy` arrays).

## Demo


## Install
`pip install imspect` for Python devs

`cargo install imspect` for Rust devs (CLI only)

## Usage

### Python interpreter\debugger
```Python
from imspect import imspect
import numpy as np

# examples of acceptable images
img1 = np.empty((60, 100, 3), dtype=np.uint8)
img2 = np.zeros((60, 100), dtype=np.uint8) + 255

imspect(img1 , img2)
```

### CLI
`imspect path/to/image.png path/to/array.npy`

