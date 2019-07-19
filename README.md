# Colliberator
Colliberator is a work-in-progress color library.

I'm developing this for my own purposes and projects, but I hope eventually it
will be broad and stable enough to be published on
[crates.io](https://crates.io).

Currently it supports RGB and HSV colors with optional alpha-channels.
The colors are generic over their colorspace, and we can transform colors
between linear and sRGB colorspaces.

There are other little things as well, such as a relative luminance function,
an ANSI-terminal text coloration function and a simple color shade
classification function.
