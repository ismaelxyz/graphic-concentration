#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# helper class for interactive object motion

import sys
from OpenGL.GLUT import *
from OpenGL.GL import *
from OpenGL.GLU import *


class InteractionMatrix (object):
    """Class holding a matrix representing a rigid transformation.

    The current OpenGL is read into an internal variable and
    updated using rotations and translations given by
    user interaction."""

    def __init__(self):
        self.__currentMatrix = None
        self.reset()

    def reset(self):
        """Initialise internal matrix with identity"""
        glPushMatrix()
        glLoadIdentity()
        self.__currentMatrix = glGetFloatv(GL_MODELVIEW_MATRIX)
        glPopMatrix()

    def addTranslation(self, tx, ty, tz):
        """Concatenate the internal matrix with a translation matrix"""
        glPushMatrix()
        glLoadIdentity()
        glTranslatef(tx, ty, tz)
        glMultMatrixf(self.__currentMatrix)
        self.__currentMatrix = glGetFloatv(GL_MODELVIEW_MATRIX)
        glPopMatrix()

    def addRotation(self, ang, rx, ry, rz):
        """Concatenate the internal matrix with a translation matrix"""
        glPushMatrix()
        glLoadIdentity()
        glRotatef(ang, rx, ry, rz)
        glMultMatrixf(self.__currentMatrix)
        self.__currentMatrix = glGetFloatv(GL_MODELVIEW_MATRIX)
        glPopMatrix()

    def getCurrentMatrix(self):
        return self.__currentMatrix


def main():
    glutInit(sys.argv)
    glutCreateWindow(sys.argv[0])
    m = InteractionMatrix()
    print(m.getCurrentMatrix())
    m.addTranslation(1, 2, 3)
    print(m.getCurrentMatrix())
    m.addRotation(30, 0, 0, 1)
    print(m.getCurrentMatrix())
    m.addTranslation(1, 2, 3)
    print(m.getCurrentMatrix())


if __name__ == '__main__':
    main()
