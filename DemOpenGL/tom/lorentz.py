#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from OpenGL.GL import *
from OpenGL.Tk import *
import tkinter
from numpy import *
import sys

n, dt = 2000, 0.01
x, y, z = 0.01, 0.01, 0.01
frac = -1.0 * (8.0/3.0)
a = array((n, 3), 'd')


def lorentz(gl, x, y, z, n=2000, dt=0.01):
    """Generate Lorentz attractor.  Put graphic in a graphical object"""
    gl.grob = glGenLists(1)
    glNewList(gl.grob, GL_COMPILE)
    try:
        glDisable(GL_LIGHTING)
        glBegin(GL_LINE_STRIP)
        try:
            glVertex3d(x, y, z)
            frac = -1.0 * (8.0/3.0)
            for i in range(0, n):
                xp = x + (-10.0 * x * dt + 10.0 * y * dt)
                yp = y + (28.0 * x * dt - y * dt - x * dt * z * dt)
                zp = z + (frac * z * dt + x * dt * y * dt)
                x = xp
                y = yp
                z = zp
                glVertex3d(x, y, z)
        finally:
            glEnd()

        glEnable(GL_LIGHTING)
    finally:
        glEndList()


def redraw(gl):
    """The main scene redraw function."""

    # Clear the background and depth buffer.
    glClearColor(1., 0., 1., 0.)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glColor3f(1., 1., 1.)
    glCallList(gl.grob)


def main():
    # Create the opengl widget here.
    gl = Opengl(None, width=400, height=400, double=1)

    # Register the redraw procedure for the widget.
    gl.redraw = redraw

    gl.pack(side='top', expand=1, fill='both')
    gl.set_centerpoint(0., 0., 2000.)
    gl.set_eyepoint(13000.)

    gl.far = 600000.

    lorentz(gl, 0.01, 0.01, 0.01)

    # Enter the tk mainloop.
    tkinter.mainloop()


if __name__ == "__main__":
    main()
