from setuptools import setup

setup(
    name="image_explorer",
    version="0.0.1",
    packages=["image_explorer"],
    url="https://github.com/SEBROVATER/image_explorer",
    license="MIT",
    author="SEBROVATER",
    author_email="sebrovskiy.k@gmail.com",
    install_requires=[
        "dearpygui>=1.9.1",
        "numpy>=1.25.0",
        "opencv-python>=4.7.0.72",
    ],
)
