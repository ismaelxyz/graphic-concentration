#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from time import sleep
import sys
import array
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *

vertices = array.array('f', [-1, -1, 1, -1, 1, 1, 1, 1, 1, 1, -1, 1,
                             -1, -1, -1, -1, 1, -1,  1, 1, -1, 1, -1, -1])

colors = array.array('f', [0, 0, 0,  1, 0, 0,  1, 1, 0,  0, 1, 0,
                           0, 0, 1,  1, 0, 1,  1, 1, 1,  0, 1, 1])

cIndices = array.array('B', [0, 3, 2, 1,  2, 3, 7, 6,  0, 4, 7, 3,
                             1, 2, 6, 5,  4, 5, 6, 7,  0, 1, 5, 4])

animationAngle = 0.0
frameRate = 25


def animationStep():
    global animationAngle
    global frameRate
    animationAngle += 2
    while animationAngle > 360:
        animationAngle -= 360
    sleep(1 / float(frameRate))
    glutPostRedisplay()


def display():
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glOrtho(-2, 2, -2, 2, -2, 2)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    glRotatef(animationAngle, 1, 1, 1)
    glEnableClientState(GL_COLOR_ARRAY)
    glEnableClientState(GL_VERTEX_ARRAY)
    glColorPointer(3, GL_FLOAT, 0, colors.tobytes())
    glVertexPointer(3, GL_FLOAT, 0, vertices.tobytes())
    glDrawElements(GL_QUADS, 24, GL_UNSIGNED_BYTE, cIndices.tobytes())
    glDisableClientState(GL_COLOR_ARRAY)
    glDisableClientState(GL_VERTEX_ARRAY)
    glutSwapBuffers()


def init():
    if not (glColorPointer and glVertexPointer and glDrawElements):
        print(''' Error: no vertex array support''')
        sys.exit()
    glClearColor(0, 0, 0, 0)
    glEnable(GL_DEPTH_TEST)
    glShadeModel(GL_SMOOTH)


def main():
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH)
    glutInitWindowSize(250, 250)
    glutInitWindowPosition(100, 100)
    glutCreateWindow(sys.argv[0])
    init()
    glutDisplayFunc(display)
    glutIdleFunc(animationStep)
    glutMainLoop()


if __name__ == '__main__':
    main()
