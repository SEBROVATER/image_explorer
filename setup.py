from setuptools import setup

setup(
    name="image_inspector",
    version="0.0.7",
    packages=["image_inspector"],
    url="https://github.com/SEBROVATER/image_explorer",
    license="MIT",
    author="SEBROVATER",
    author_email="sebrovskiy.k@gmail.com",
    install_requires=[
        "dearpygui",
        "numpy",
        "opencv-python",
    ],
)
