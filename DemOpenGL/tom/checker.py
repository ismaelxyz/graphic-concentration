#!/usr/bin/python

from OpenGL.GL import *
from OpenGL.GLUT import *
from OpenGL.Tk import *
import sys

def redraw(_):
    glClearColor(1, 0, 1, 0)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glColor3f(0, 1, 0)
    # draw checkerboard
    N = 4
    glDisable(GL_LIGHTING)
    for x in range(-N, N):
        for y in range(-N, N):
            if (x + y) % 2 == 0:
                glColor3f(1, 1, 1)
            else:
                glColor3f(0, 0, 0)
            glRectf(x, y, x + 1, y + 1)
    glEnable(GL_LIGHTING)

    glPushMatrix()
    glTranslatef(0., 0., 1.)
    glutSolidSphere(1.0, 20, 20)
    glPopMatrix()


def main():
    frame = Frame()
    frame.pack(side='top')
    gl = Opengl(width=200, height=200, double=1, depth=1)
    glutInit([])
    gl.redraw = redraw
    quit = Button(frame, text='Quit', command=sys.exit)
    quit.pack({'side': 'top', 'side': 'left'})
    help = Button(frame, text='Help', command=gl.help)
    help.pack({'side': 'top', 'side': 'left'})
    reset = Button(frame, text='Reset', command=gl.reset)
    reset.pack({'side': 'top', 'side': 'left'})
    gl.pack(side='top', expand=1, fill='both')
    gl.set_eyepoint(20.)
    gl.mainloop()


if __name__ == '__main__':
    main()
