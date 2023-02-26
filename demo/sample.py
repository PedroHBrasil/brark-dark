"""Sample file for theme visualization on Python code.
"""
import os
from pathlib import Path


class Sample(object):
    """Sample class for theme visualization on Python code.
    """

    def __init__(self, name):
        self.name = name

    @property
    def name(self):
        """Name of the sample."""
        return self._name

    @name.setter
    def name(self, name):
        """Set the name of the sample."""
        self._name = name

    def hello_world(self):
        """Print hello world message."""
        print("Hello World from {}".format(self.name))


def run():
    """Run the sample."""
    sample = Sample("Python")
    sample.hello_world()


if __name__ == "__main__":
    run()
