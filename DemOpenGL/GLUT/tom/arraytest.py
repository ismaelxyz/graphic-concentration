from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GLUT import *
import time
import sys

from numpy import arange, cos, sin, pi as PI, ravel, transpose, reshape, \
    concatenate
from numpy.random import random

n = 50


def shuffle(a, b):
    return ravel(transpose(reshape(concatenate([a, b]), (2, len(a)))))


def buildArrays():
    a = arange(0, n)
    vertex = shuffle(cos(2*PI*a/n), sin(2*PI*a/n))
    vertex.shape = (n, 2)
    color = random(n*3)
    color.shape = (n, 3)
    return vertex, color


vertex, color = buildArrays()


def drawArrays():
    glVertexPointerd(vertex)
    glColorPointerd(color)
    glEnableClientState(GL_VERTEX_ARRAY)
    glEnableClientState(GL_COLOR_ARRAY)
    glDisable(GL_LIGHTING)
    try:
        glDrawArrays(GL_LINE_LOOP, 0, n)
    finally:
        glEnable(GL_LIGHTING)


def display(swap=1, clear=1):
    """Callback function for displaying the scene

    This defines a unit-square environment in which to draw,
    i.e. width is one drawing unit, as is height
    """
    glClearColor(0.5, 0.5, 0.5, 0)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)

    # establish the projection matrix (perspective)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glOrtho(-1, 1, -1, 1, -1, 1)

    # and then the model view matrix
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    rotation()

    drawArrays()

    glutSwapBuffers()


def idle():
    glutPostRedisplay()


starttime = time.time()


def rotation(period=10):
    """Do rotation of the scene at given rate"""
    angle = (((time.time() - starttime) % period) / period) * 360
    glRotate(angle, 0, 1, 0)
    return angle


def main():
    print('You should see a polynomial curve rotating about the origin.')
    import sys
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_DEPTH)
    glutCreateWindow('Array Drawing Demo')
    glutDisplayFunc(display)
    glutIdleFunc(display)
    # note need to do this to properly render faceted geometry
    glutMainLoop()


if __name__ == '__main__':
    main()
