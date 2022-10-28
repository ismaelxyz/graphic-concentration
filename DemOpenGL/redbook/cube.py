#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
    This program demonstrates a single modeling transformation,
    glScalef() and a single viewing transformation, gluLookAt().
    A wireframe cube is rendered.
"""

import sys
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *
window = None


def init():
    glClearColor(0.0, 0.0, 0.0, 0.0)
    glShadeModel(GL_FLAT)


def display():
    glClear(GL_COLOR_BUFFER_BIT)
    glColor3f(1.0, 1.0, 1.0)
    glLoadIdentity()             # clear the matrix
    # viewing transformation
    gluLookAt(0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    glScalef(1.0, 2.0, 1.0)      # modeling transformation
    glutWireCube(1.0)
    glFlush()


def reshape(w, h):
    glViewport(0, 0, w, h)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glFrustum(-1.0, 1.0, -1.0, 1.0, 1.5, 20.0)
    glMatrixMode(GL_MODELVIEW)


def keyboard(key, x, y):
    if key == b'\x1b':
        glutDestroyWindow(window)


def main():
    global window
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB)
    glutInitWindowSize(500, 500)
    glutInitWindowPosition(100, 100)
    window = glutCreateWindow('Cube')
    init()
    glutDisplayFunc(display)
    glutReshapeFunc(reshape)
    glutKeyboardFunc(keyboard)
    glutMainLoop()


if __name__ == '__main__':
    main()
