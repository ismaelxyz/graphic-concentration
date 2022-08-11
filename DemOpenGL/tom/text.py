#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *
import tkinter
import sys

try:
    from logo import define_logo
except:
    from .logo import define_logo

def redraw(gl):
    if gl.grob == -1:
        gl.grob = glGenLists(1)
        glNewList(gl.grob, GL_COMPILE_AND_EXECUTE)
        glMaterialfv(GL_FRONT, GL_DIFFUSE, [1., 1., 0., 0.])
        define_logo()
        glEndList()

        gl.autospin = 1

        gl.xspin = 1
        gl.yspin = 2

        gl.update()

        gl.after(10, gl.do_AutoSpin)

    else:

        glCallList(gl.grob)

def main():
    gl = Opengl(None, width=400, height=200, double=1, depth=1)
    gl.pack(expand=1, fill='both')

    gl.redraw = redraw
    gl.set_centerpoint(30., 2., 0.)
    gl.set_eyepoint(80.)

    gl.grob = -1

    gl.autospin_allowed = 1

    # Enter the tk mainloop.

    tkinter.mainloop()

if __name__ == "__main__":
    main()
