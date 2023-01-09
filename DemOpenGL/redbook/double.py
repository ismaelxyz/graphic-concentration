#!/usr/bin/env python3

'''
    double.c from the Redbook examples.  
    Converted to Python by Jason L. Petrone 6/00

    This is a simple double buffered program.
    Pressing the left mouse button rotates the rectangle.
    Pressing the middle mouse button stops the rotation.
'''


import sys
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *
spin = 0.0


def display():
    glClear(GL_COLOR_BUFFER_BIT)
    glPushMatrix()
    glRotatef(spin, 0.0, 0.0, 1.0)
    glColor3f(1.0, 1.0, 1.0)
    glRectf(-25.0, -25.0, 25.0, 25.0)
    glPopMatrix()
    glutSwapBuffers()


def spin_display():
    global spin
    spin = spin + 2.0
    if (spin > 360.0):
        spin = spin - 360.0
    glutPostRedisplay()


def init():
    glClearColor(0.0, 0.0, 0.0, 0.0)
    glShadeModel(GL_FLAT)


def reshape(w, h):
    glViewport(0, 0, w, h)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glOrtho(-50.0, 50.0, -50.0, 50.0, -1.0, 1.0)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()


def mouse(button, state, x, y):
    if button == GLUT_LEFT_BUTTON:
        if (state == GLUT_DOWN):
            glutIdleFunc(spin_display)
    elif button == GLUT_MIDDLE_BUTTON or button == GLUT_RIGHT_BUTTON:
        if (state == GLUT_DOWN):
            glutIdleFunc(None)


def main():
    """
        Request double buffer display mode.
        Register mouse input callback functions
    """
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB)
    glutInitWindowSize(250, 250)
    glutInitWindowPosition(100, 100)
    glutCreateWindow('Double')
    init()
    glutDisplayFunc(display)
    glutReshapeFunc(reshape)
    glutMouseFunc(mouse)
    glutMainLoop()


if __name__ == '__main__':
    main()
