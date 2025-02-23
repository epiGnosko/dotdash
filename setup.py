from setuptools import setup, find_packages

setup(
    name='dotdash',
    version='1.0',
    description='A CLI tool to translate between Morse code and text.',
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    author='Gurmukh Singh',
    url='https://github.com/Gurmukh-Singh-4253/dotdash'
    # may or may not add author_email
    packages=find_packages(),
    entry_points={
        'console_scripts': [
            'dotdash=dotdash.main:main',
        ],
    },
    install_requires=[
        'argparse',
    ],
)
