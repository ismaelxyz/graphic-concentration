#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.Tk import *


def redraw(o):
    glClearColor(0.5, 0.5, 0.5, 0)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glOrtho(0, 1, 0, 1, 0, 1)
    glDisable(GL_LIGHTING)
    glBegin(GL_LINES)
    glColor3f(1, 1, 0)
    glVertex2f(0, 0)
    glColor3f(1, 0, 1)
    glVertex2f(1, 1)
    glColor3f(1, 0, 0)
    glVertex2f(1, 0)
    glColor3f(0, 0, 1)
    glVertex2f(0, 1)
    glEnd()
    glEnable(GL_LIGHTING)


def main():
    gl = Opengl(width=400, height=400, double=1)
    gl.redraw = redraw
    gl.pack(side='top', expand=1, fill='both')
    gl.mainloop()


if __name__ == '__main__':
    main()
