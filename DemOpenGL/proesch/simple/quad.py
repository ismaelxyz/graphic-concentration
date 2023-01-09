from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *

import sys

from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *


def display():
    """OpenGL display method."""
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glColor3f(1, 1, 1)
    glBegin(GL_QUADS)
    glVertex3f(-0.5, 0.5, 0)
    glVertex3f(-0.5, -0.5, 0)
    glVertex3f(0.5, -0.5, 0)
    glVertex3f(0.5, 0.5, 0)
    glEnd()
    glFlush()


def init():
    """OpenGL/glut init method."""
    glClearColor(0, 0, 0, 0)
    glShadeModel(GL_SMOOTH)


def main():
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB)
    glutInitWindowSize(250, 250)
    glutInitWindowPosition(100, 100)
    glutCreateWindow(sys.argv[0])
    init()
    glutDisplayFunc(display)
    glutMainLoop()


if __name__ == '__main__':
    main()
