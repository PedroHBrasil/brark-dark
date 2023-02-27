import math
import os
import sys
from typing import List, Optional

# Variables and Types
greeting = "Hello, world!"
age = 42
pi = 3.14159
is_rust_fan = True

# Lists
numbers = [1, 2, 3, 4, 5]
squares = [n**2 for n in numbers]
evens = [n for n in numbers if n % 2 == 0]

# Dictionaries
person = {"name": "Alice", "age": 30, "location": "New York"}
person["occupation"] = "Engineer"

# Functions


def say_hello(name: str) -> None:
    """Prints a greeting to the given name."""
    print(f"Hello, {name}!")


def square_root(x: float) -> Optional[float]:
    """
    Returns the square root of a number, or None if the number is negative.

    Args:
        x: The number to take the square root of.

    Returns:
        The square root of the number, or None if the number is negative.
    """
    if x < 0:
        return None
    else:
        return math.sqrt(x)

# Classes and Inheritance


class Animal:
    def __init__(self, name: str, species: str):
        self.name = name
        self.species = species

    def speak(self) -> None:
        print("The animal makes a sound.")


class Cat(Animal):
    def __init__(self, name: str):
        super().__init__(name, "cat")

    def speak(self) -> None:
        print("Meow.")

# Error Handling with Exceptions


def divide(x: float, y: float) -> float:
    """
    Divides two numbers and returns the result.

    Args:
        x: The numerator.
        y: The denominator.

    Raises:
        ValueError: If the denominator is zero.

    Returns:
        The result of the division.
    """
    if y == 0:
        raise ValueError("Cannot divide by zero.")
    else:
        return x / y

# File I/O


def read_file_contents(file_path: str) -> Optional[str]:
    """
    Reads the contents of a file and returns them as a string.

    Args:
        file_path: The path to the file to read.

    Returns:
        The contents of the file as a string, or None if the file cannot be read.
    """
    try:
        with open(file_path, "r") as f:
            return f.read()
    except OSError as e:
        print(f"Error reading file: {e}")
        return None

# Decorators


def benchmark(func):
    """
    A decorator that prints the time taken to execute a function.
    """
    def wrapper(*args, **kwargs):
        import time
        start_time = time.time()
        result = func(*args, **kwargs)
        end_time = time.time()
        print(f"Time taken: {end_time - start_time:.2f} seconds")
        return result
    return wrapper


@benchmark
def expensive_function() -> None:
    """
    A function that takes a long time to execute.
    """
    for i in range(10000000):
        pass

# Unit Tests


def test_square_root() -> None:
    assert square_root(4) == 2.0
    assert square_root(-1) is None

# Main function


def main() -> None:
    """
    The main function
    """
    say_hello("Alice")
    print(square_root(4))
    print(square_root(-1))
    a = Animal("Bob", "dog")
    a.speak()
    c = Cat("Fluffy")
    c.speak()
    try:
        print(divide(10, 2))
        print(divide(10, 0))
    except ValueError as e:
        print(e)
    file_contents = read_file_contents("example.txt")
    if file_contents is not None:
        print(file_contents)
    expensive_function()
    test_square_root()


if __name__ == "__main__":
    main()
