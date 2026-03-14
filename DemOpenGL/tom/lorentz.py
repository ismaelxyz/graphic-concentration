#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import OpenGL.GL as gl
import OpenGL.Tk as tk
import tkinter
import numpy as np
import sys

n, dt = 2000, 0.01
x, y, z = 0.01, 0.01, 0.01
frac = -1.0 * (8.0 / 3.0)
a = np.array((n, 3), "d")


def lorentz(gltk, x, y, z, n=2000, dt=0.01):
    """Generate Lorentz attractor.  Put graphic in a graphical object"""
    gltk.grob = gl.glGenLists(1)
    gl.glNewList(gltk.grob, gl.GL_COMPILE)
    try:
        gl.glDisable(gl.GL_LIGHTING)
        gl.glBegin(gl.GL_LINE_STRIP)
        try:
            gl.glVertex3d(x, y, z)
            frac = -1.0 * (8.0 / 3.0)
            for i in range(0, n):
                xp = x + (-10.0 * x * dt + 10.0 * y * dt)
                yp = y + (28.0 * x * dt - y * dt - x * dt * z * dt)
                zp = z + (frac * z * dt + x * dt * y * dt)
                x = xp
                y = yp
                z = zp
                gl.glVertex3d(x, y, z)
        finally:
            gl.glEnd()  # type: ignore[call-arg]

        gl.glEnable(gl.GL_LIGHTING)
    finally:
        gl.glEndList()


def redraw(gltk):
    """The main scene redraw function."""

    # Clear the background and depth buffer.
    gl.glClearColor(1.0, 0.0, 1.0, 0.0)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glColor3f(1.0, 1.0, 1.0)
    gl.glCallList(gltk.grob)


def main():
    # Create the opengl widget here.
    gltk = tk.Opengl(None, width=400, height=400, double=1)

    # Register the redraw procedure for the widget.
    gltk.redraw = redraw

    gltk.pack(side="top", expand=1, fill="both")
    gltk.set_centerpoint(0.0, 0.0, 2000.0)
    gltk.set_eyepoint(13000.0)

    gltk.far = 600000.0

    lorentz(gltk, 0.01, 0.01, 0.01)

    # Enter the tk mainloop.
    tkinter.mainloop()


if __name__ == "__main__":
    main()
