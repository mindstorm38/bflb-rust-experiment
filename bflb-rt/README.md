# Embedded Runtime

This rust crate aim to provides a common runtime crate to import for your bare-metal projects.
The goal is to provide an abstraction for runtime on different chips. It acts as a low-level
operating system where you program a single process that will have a low-level access, you'll
give an entry process that will be started just after a little startup code. This crate also
provides IRQ and exceptions handling.
