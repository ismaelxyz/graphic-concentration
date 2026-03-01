#!/usr/bin/python


import sys
from OpenGL.GL import (
    glClearDepth,
    glEnable,
    glClearColor,
    glShadeModel,
    glColorMaterial,
    glMatrixMode,
    glFrustum,
    glLightfv,
    GL_DEPTH_TEST,
    GL_SMOOTH,
    GL_PROJECTION,
    GL_MODELVIEW,
    GL_LIGHT0,
    GL_DIFFUSE,
    GL_LIGHT1,
    GL_COLOR_MATERIAL,
    GL_FRONT_AND_BACK,
    GL_LIGHTING,
    GL_POSITION,
)

from OpenGL.GLUT import (
    glutInit,
    glutInitDisplayMode,
    glutCreateWindow,
    glutDisplayFunc,
    glutMotionFunc,
    glutCreateMenu,
    glutAddMenuEntry,
    glutAttachMenu,
    glutPostRedisplay,
    glutMainLoop,
    GLUT_DOUBLE,
    GLUT_RGB,
    GLUT_DEPTH,
    GLUT_RIGHT_BUTTON,
)


class Maintest:
    # set up a light
    light_one_position = (40.0, 40, 100.0, 0.0)
    light_one_color = (0.99, 0.99, 0.99, 1.0)

    light_two_position = (-40.0, 40, 100.0, 0.0)
    light_two_color = (0.99, 0.99, 0.99, 1.0)

    def __init__(self, last_x: float = 0.0, last_y: float = 0.0):
        self.last_x = last_x
        self.last_y = last_y

    def mouse_motion(self, x, y):
        """Called when the mouse moves with a button down. We use this to rotate the object."""
        self.last_x = x
        self.last_y = y
        glutPostRedisplay()

    def join_style(self, code: int = 0):
        """Called when a menu item is selected. We use this to exit the program."""
        sys.exit(code)

    def main_loop(self, draw_callback):
        """Set up the OpenGL context and enter the main loop."""
        # initialize glut
        glutInit(sys.argv)
        glutInitDisplayMode(int(GLUT_DOUBLE) | int(GLUT_RGB) | int(GLUT_DEPTH))
        glutCreateWindow("Basic Demo")
        glutDisplayFunc(draw_callback)
        glutMotionFunc(self.mouse_motion)

        # create popup menu
        glutCreateMenu(self.join_style)
        glutAddMenuEntry("Exit", 99)
        glutAttachMenu(GLUT_RIGHT_BUTTON)

        # initialize GL
        glClearDepth(1.0)
        glEnable(GL_DEPTH_TEST)
        glClearColor(0.0, 0.0, 0.0, 0.0)
        glShadeModel(GL_SMOOTH)

        glMatrixMode(GL_PROJECTION)
        # roughly, measured in centimeters
        glFrustum(-9.0, 9.0, -9.0, 9.0, 50.0, 150.0)
        glMatrixMode(GL_MODELVIEW)

        # initialize lighting
        glLightfv(GL_LIGHT0, GL_POSITION, Maintest.light_one_position)
        glLightfv(GL_LIGHT0, GL_DIFFUSE, Maintest.light_one_color)
        glEnable(GL_LIGHT0)
        glLightfv(GL_LIGHT1, GL_POSITION, Maintest.light_two_position)
        glLightfv(GL_LIGHT1, GL_DIFFUSE, Maintest.light_two_color)
        glEnable(GL_LIGHT1)
        glEnable(GL_LIGHTING)
        glColorMaterial(GL_FRONT_AND_BACK, GL_DIFFUSE)
        glEnable(GL_COLOR_MATERIAL)

        glutMainLoop()
