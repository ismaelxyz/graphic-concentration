#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *


def redraw(o):
    glClearColor(0, 0, 1, 0)
    glClear(GL_COLOR_BUFFER_BIT)


def main():
    gl = Opengl(width=400, height=400, double=1)
    gl.redraw = redraw
    gl.pack(side='top', expand=1, fill='both')
    gl.mainloop()


if __name__ == '__main__':
    main()