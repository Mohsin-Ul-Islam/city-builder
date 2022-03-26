|ci|

.. |ci| image:: https://github.com/Mohsin-Ul-Islam/city-builder/actions/workflows/ci.yml/badge.svg
   :target:

.. _SFML: http://www.sfml-dev.org/download.php
.. _CSFML: http://www.sfml-dev.org/download/csfml/

City Builder Simulator
######################

This project is a city builder simulation game. This is my hobby project. See Contributing for making the project awesome.

Building the project
====================

To build the project make sure to install `SFML` and `CSFML` first. On Debian based Linux (Ubuntu) run:

``sudo apt install libcsfml-dev libsfml-dev``

Then build with cargo as usual.

``cargo build``

And run the project

``cargo run``

Contributing
============

If you have any suggestions for improvements or bug fixes please create an issue. Create pull requests for features on the main branch. We will review it.

Make sure to format and lint the code before creating PR

``cargo fmt --all``

For linting

``cargo clippy``
