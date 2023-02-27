// Define basic data types
int num1 = 42;
double num2 = 3.14;
String str1 = "Hello, world!";
bool flag1 = true;

// Define complex data types
List<int> list1 = [1, 2, 3];
Map<String, int> map1 = {"one": 1, "two": 2, "three": 3};
Set<double> set1 = {1.0, 2.0, 3.0};

// Control flow structures
void controlFlow() {
  // If-else
  if (num1 > 0) {
    print("num1 is positive");
  } else if (num1 == 0) {
    print("num1 is zero");
  } else {
    print("num1 is negative");
  }

  // Switch
  switch (str1) {
    case "Hello":
      print("The string is 'Hello'");
      break;
    case "world":
      print("The string is 'world'");
      break;
    default:
      print("The string is something else");
  }

  // For loop
  for (int i = 0; i < list1.length; i++) {
    print(list1[i]);
  }

  // While loop
  int i = 0;
  while (i < set1.length) {
    print(set1.elementAt(i));
    i++;
  }

  // Do-while loop
  i = 0;
  do {
    print(list1[i]);
    i++;
  } while (i < list1.length);

  // Try-catch
  try {
    int result = num1 ~/ 0;
  } catch (e) {
    print("Error: $e");
  }
}

// Define a function with parameters and return type
int add(int a, int b) {
  return a + b;
}

// Define an anonymous function
Function multiply = (int a, int b) => a * b;

// Define a closure
Function makeAdder(int addBy) {
  return (int i) => addBy + i;
}

// Define a class
class Person {
  String name;
  int age;

  // Constructor
  Person(this.name, this.age);

  // Method
  void sayHello() {
    print("Hello, my name is $name and I am $age years old.");
  }

  // Getter and setter
  String get getName => name;
  set setAge(int newAge) => age = newAge;
}

// Define a subclass that extends the Person class and implements an interface
abstract class Greetable {
  void greet();
}

class Student extends Person implements Greetable {
  String major;

  // Constructor
  Student(String name, int age, this.major) : super(name, age);

  // Method
  @override
  void sayHello() {
    super.sayHello();
    print("I am majoring in $major.");
  }

  // Interface method implementation
  @override
  void greet() {
    print("Hi, I'm a student!");
  }
}

// Define a mixin
mixin MixinExample {
  void mixinMethod() {
    print("This is a method defined in the mixin.");
  }
}

// Define a generic class and function
class Box<T> {
  T value;
  Box(this.value);
}

T genericFunction<T>(T arg) {
  return arg;
}

// Define a library and import another library
library my_library;

import 'dart:math';

// Null safety
int? nullableInt;
int non

import 'dart:async';

class Person {
  final String name;
  final int age;
  final String? email;

  const Person({
    required this.name,
    required this.age,
    this.email,
  });

  String get fullName => '$name Doe';

  void greet() {
    print('Hello, my name is $fullName');
  }

  @override
  String toString() {
    return 'Person(name: $name, age: $age, email: $email)';
  }
}

mixin DanceMoves {
  void dance() {
    print('Doing the dance moves!');
  }
}

abstract class Musician {
  String get instrument;
  void play();
}

class GuitarPlayer implements Musician {
  @override
  String get instrument => 'guitar';

  @override
  void play() {
    print('Playing the guitar...');
  }
}

enum Genre {
  rock,
  pop,
  jazz,
}

extension GenreExtension on Genre {
  String get name {
    return this.toString().split('.').last;
  }
}

Future<void> main() async {
  final person = const Person(name: 'John', age: 30);
  person.greet();

  final guitarPlayer = GuitarPlayer();
  guitarPlayer.play();

  final genre = Genre.rock;
  print('The genre is ${genre.name}');
}
