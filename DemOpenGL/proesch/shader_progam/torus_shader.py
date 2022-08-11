#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Demonstration of the shaderProgram:
Animated torus with procedural texture.
"""

from time import sleep
import sys
from shader_prog import ShaderProgram
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GL.ARB.shader_objects import *
from OpenGL.GL.ARB.fragment_shader import *
from OpenGL.GL.ARB.vertex_shader import *
import logging

frameRate = 25
time = 0.0
sP = None
torusList = None

def animationStep():
    """Update animated parameters."""
    global frameRate, time, sP
    
    time += 0.05
    if sP and sP.enable():
        glUseProgramObjectARB(1)
        glUniform1fARB(sP.indexOfUniformVariable("Time"), time)
    sleep(1 / float(frameRate))
    glutPostRedisplay()


def display():
    """Glut display function."""
    if not torusList:
        init()
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    xSize, ySize = glutGet(GLUT_WINDOW_WIDTH), glutGet(GLUT_WINDOW_HEIGHT)
    gluPerspective(60, float(xSize) / float(ySize), 0.1, 50)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    glTranslatef(0, 0, -4)
    glRotatef(130, 1, 0, 0)
    glCallList(torusList)
    glutSwapBuffers()


def initShaders():
    """Initialise shaderProg object."""
    global sP
    sP = ShaderProgram()
    sP.addShader(GL_VERTEX_SHADER_ARB, "brick.vert")
    sP.addShader(GL_FRAGMENT_SHADER_ARB, "brick.frag")
    sP.linkShaders()
    sP.enable()
    glUniform1fARB(sP.indexOfUniformVariable("Amplitude"), 0.3)
    glUniform3fvARB(sP.indexOfUniformVariable("LightPosition"), 1,
                    (0.0, 0.0, 3.0))
    glUniform3fvARB(sP.indexOfUniformVariable("BrickColor"), 1,
                    (1.0, 0.3, 0.2))
    glUniform3fvARB(sP.indexOfUniformVariable("MortarColor"), 1,
                    (0.85, 0.86, 0.84))
    glUniform2fvARB(sP.indexOfUniformVariable("BrickSize"), 1,
                    (0.3, 0.15))
    glUniform2fvARB(sP.indexOfUniformVariable("BrickPct"), 1,
                    (0.9, 0.85))


def init():
    """Glut init function."""
    global torusList
    glClearColor(0.3, 0.3, 0.3, 1)
    glEnable(GL_DEPTH_TEST)
    glShadeModel(GL_SMOOTH)
    torusList = glGenLists(1)
    glNewList(torusList, GL_COMPILE)
    glutSolidTorus(0.5, 1, 40, 50)
    glEndList()
    initShaders()


def main():
    logging.basicConfig()
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB | GLUT_DEPTH)
    glutInitWindowSize(250, 250)
    glutInitWindowPosition(100, 100)
    glutCreateWindow(sys.argv[0])
    glutDisplayFunc(display)
    glutIdleFunc(animationStep)
    glutMainLoop()


if __name__ == "__main__":
    main()
