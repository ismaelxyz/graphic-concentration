#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
    Plot a circle using NURBS
"""

import sys
import math
from time import sleep
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *

animationAngle = 0.0
frameRate = 25
animationTime = 0
degree = 3
s2 = math.sqrt(2)/2.0

# Initialise circle control points.
circlePoints = [
    [0.0, 1.0, 0.0, 1.0],
    [s2, s2, 0.0, s2],
    [1.0, 0.0, 0.0, 1.0],
    [s2, -s2, 0.0, s2],
    [0.0, -1.0, 0.0, 1.0],
    [-s2, -s2, 0.0, s2],
    [-1.0, 0.0, 0.0, 1.0],
    [-s2, s2, 0.0, s2],
]

# make sure circle is closed properly
circlePoints = circlePoints + [circlePoints[0], circlePoints[1]]

# initialise circle knots
circleKnots = [0.0] + \
    [float(i/2) for i in range(len(circlePoints) + degree - 1)]
nurb = None
samplingTolerance = 1.0


def animationStep():
    """Update animated parameters"""
    global animationAngle, frameRate

    animationAngle += 0.3
    while animationAngle > 360:
        animationAngle -= 360

    sleep(1 / float(frameRate))
    glutPostRedisplay()


def display():
    global circlePoints, circleKnots, nurb

    glClear(GL_COLOR_BUFFER_BIT)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    xSize, ySize = glutGet(GLUT_WINDOW_WIDTH), glutGet(GLUT_WINDOW_HEIGHT)
    gluPerspective(60, float(xSize) / float(ySize), 0.1, 50)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    glTranslatef(0, 0, -2)
    glRotatef(animationAngle, 0, 0, 1)

    glColor3f(0, 1, 0)
    glBegin(GL_LINE_STRIP)
    for coord in circlePoints:
        glVertex3f(coord[0], coord[1], coord[2])

    glEnd()
    glColor3f(1, 1, 1)
    gluBeginCurve(nurb)
    gluNurbsCurve(nurb, circleKnots, circlePoints, GL_MAP1_VERTEX_4)
    gluEndCurve(nurb)
    glutSwapBuffers()


def init():
    """Glut init function."""
    global nurb, samplingTolerance

    glClearColor(0, 0, 0, 0)
    nurb = gluNewNurbsRenderer()
    glLineWidth(2.0)
    gluNurbsProperty(nurb, GLU_SAMPLING_TOLERANCE, samplingTolerance)


def main():
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB)
    glutInitWindowSize(250, 250)
    glutInitWindowPosition(100, 100)
    glutCreateWindow(sys.argv[0])
    init()
    glutDisplayFunc(display)
    glutIdleFunc(animationStep)
    glutMainLoop()


if __name__ == '__main__':
    main()
