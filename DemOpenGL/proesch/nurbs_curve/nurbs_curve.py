#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Plot a curve with control points positioned on a circle
using NURBS.
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
# position points on a circle
angleNum = 5
radius = 1
angles = [2*math.pi*float(i)/angleNum for i in range(angleNum)]

circlePoints = [
    [radius*math.cos(theta), radius*math.sin(theta), 0] for theta in angles]

# make sure the curve is closed properly
degree = 3
for i in range(degree-1):
    circlePoints = circlePoints + [circlePoints[i]]

knotNum = len(circlePoints) + degree
circleKnots = [float(i)/(knotNum-1) for i in range(knotNum)]
nurb = None
samplingTolerance = 1.0


def animation_step():
    """Update animated parameters"""
    global animationAngle, frameRate

    animationAngle += 0.3
    while animationAngle > 360:
        animationAngle -= 360

    sleep(1 / float(frameRate))
    glutPostRedisplay()


def display():
    """Glut display function."""
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
    gluNurbsCurve(nurb, circleKnots, circlePoints, GL_MAP1_VERTEX_3)
    gluEndCurve(nurb)
    glutSwapBuffers()


def init():
    """Glut init function."""
    global nurb, samplingTolerance
    glClearColor(0, 0, 0, 0)
    glShadeModel(GL_FLAT)
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
    glutIdleFunc(animation_step)
    glutMainLoop()


if __name__ == '__main__':
    main()
