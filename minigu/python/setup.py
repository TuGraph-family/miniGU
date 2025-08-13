from setuptools import setup, find_packages

setup(
    name="minigu",
    version="0.1.0",
    packages=find_packages(),
    package_data={
        'minigu': ['*.pyd', '*.so', '*.dll'],
    },
    author="miniGU Team",
    description="A graph database for learning purposes",
    python_requires=">=3.6",
)