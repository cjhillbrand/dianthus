[![All Tests Passing](https://github.com/cjhillbrand/dianthus/actions/workflows/pull-request-build.yml/badge.svg?branch=main)](https://github.com/cjhillbrand/dianthus/actions/workflows/pull-request-build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
# Dianthus
This project is being developed due to a passion for compilers and interpreters. 
As well as a wanting to explore one of the newer programming languages, rust. <br/> <br/>
This is my first attempt at writing rust, so if there is something that looks off,
or doesn't follow particular idiomatic patterns, please feel free to point it out to me.
This project is mostly a learning experience for myself, so the more people sharing their knowledge
the more successful this project will be.

## Challenges
1. Trying to implement:
```java
public class Example {
    public static void main(String[] args)
    {
        System.out.println("Hello World");
    }
}
```
Why is this important? Brian Kernighan the author of, "A Tutorial Introduction to the Programming Language B"
referenced the "Hello World" statement for the first time and is now one of
the most popular introductory programs to write. The first milestone for this project was to follow this pattern.

I was not able to accomplish this implementation as there were to many exceptions when dealing with the "java/lang/System"
class. The time of initialization is different than other classes that are referenced in a static manner.
This class is initialized at thread creation, where all of the ouptut streams and input streams are initialized. 
Another challenge of the implementation of the example method is that I/O in the JVM is dependant on the Java Native 
implementation. The JNI as stated from the oracle docs, "Java Native Interface (JNI) is a standard programming interface 
for writing Java native methods and embedding the Java virtual machine into native applications. The primary goal is binary 
compatibility of native method libraries across all Java virtual machine implementations on a given platform." [docs](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/)
Although the JNI is wildly interesting; trying to bridge managed code and native code seems like a very hard problem.

