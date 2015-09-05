from setuptools import setup

setup(
    verson='1.0',
    name='quoridor',
    packages=["quoridor"],
    install_requires=[
        'sqlalchemy==0.9.9',
        'sphinxcontrib-httpdomain==1.3.0',
        'Sphinx==1.3.1',
    ],
)
