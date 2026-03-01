"""GLUT replacement for the original conechecker.py demonstration code

Note:
    Has no navigation code ATM.  Rather than creating two contexts,
    it creates two viewports in the same context and renders into them
    using the display functions from the other demo modules.
"""

import time
import sys
import cone
import checker
from OpenGL.GL import (
    glClear,
    glClearColor,
    glRotate,
    glViewport,
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
)
from OpenGL.GLUT import (
    glutCreateWindow,
    glutDisplayFunc,
    glutGet,
    glutIdleFunc,
    glutInit,
    glutInitDisplayMode,
    glutMainLoop,
    glutPostRedisplay,
    GLUT_DEPTH,
    GLUT_DOUBLE,
    GLUT_RGBA,
    GLUT_WINDOW_HEIGHT,
    GLUT_WINDOW_WIDTH,
)


def display():
    """Callback function for displaying the scene

    This defines a unit-square environment in which to draw,
    i.e. width is one drawing unit, as is height
    """
    glClearColor(0.5, 0.5, 0.5, 0)
    glClear(int(GL_COLOR_BUFFER_BIT) | int(GL_DEPTH_BUFFER_BIT))
    width, height = glutGet(GLUT_WINDOW_WIDTH), glutGet(GLUT_WINDOW_HEIGHT)
    halfHeight = int(height / 2.0)
    glViewport(0, halfHeight, width, halfHeight + 1)
    # glClear doesn't restrict itself to the viewport,
    # so we have to tell the child viewports not to use it...
    checker.display(swap=0, clear=0)
    glViewport(0, 0, width, halfHeight)
    cone.display(swap=1, clear=0)
    glViewport(0, 0, width, height)


def idle():
    glutPostRedisplay()


starttime = time.time()


def rotation(period=10):
    """Do rotation of the scene at given rate"""
    angle = (((time.time() - starttime) % period) / period) * 360
    glRotate(angle, 0, 1, 0)
    return angle


def main():
    print(
        "You should see two OpenGL viewports, in the top, a "
        "sphere+checker-board and in the bottom, a rotating cone."
    )

    glutInit(sys.argv)
    glutInitDisplayMode(int(GLUT_RGBA) | int(GLUT_DOUBLE) | int(GLUT_DEPTH))
    glutCreateWindow("Two-scene Demo (Cone Checker)")
    glutDisplayFunc(display)
    glutIdleFunc(display)
    # note need to do this to properly render faceted geometry
    glutMainLoop()


if __name__ == "__main__":
    main()
