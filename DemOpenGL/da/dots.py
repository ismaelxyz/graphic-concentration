#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
import os
from OpenGL.GL import *  # type: ignore
from OpenGL.GLUT import *  # type: ignore

from numpy import cos, sin, greater, choose, transpose, concatenate, array
from numpy.random import random, randint

MY_LIST = 1
NUMDOTS = 500
NUMDOTS2 = 600
MAX_AGE = 13

x = random(NUMDOTS) * 2 - 1
y = random(NUMDOTS) * 2 - 1
age = randint(0, MAX_AGE, (NUMDOTS,))
angle = 0  # in radians
delta_angle = 0.2  # in radians
move_length = 0.005  # 1.0 = screen width
move_x = move_length * cos(angle)
move_y = move_length * sin(angle)
halted = 0
window = None


def display(*_args):
    global x, y, move_x, move_y, NUMDOTS, NUMDOTS2, MAX_AGE, age

    glClearColor(0.0, 0.0, 0.0, 0.0)
    glClear(GL_COLOR_BUFFER_BIT)
    glColor3f(1.0, 1.0, 0.0)
    x = x + move_x
    y = y + move_y

    age = age + 1
    which = greater(age, MAX_AGE)
    x = choose(which, (x, random(NUMDOTS)))
    y = choose(which, (y, random(NUMDOTS)))

    age = choose(which, (age, array([0])))
    x = choose(greater(x, 1.0), (x, x - 1.0))  # very cool - wraparound
    y = choose(greater(y, 1.0), (y, y - 1.0))
    x2 = random(NUMDOTS2)
    y2 = random(NUMDOTS2)
    v = concatenate(
        (
            transpose(array([x, y])),
            transpose(array([x - 0.005, y + 0.005])),
            transpose(array([x + 0.005, y - 0.005])),
            transpose(array([x2, y2])),
        )
    )

    # Use proper vertex array setup
    glEnableClientState(GL_VERTEX_ARRAY)
    glVertexPointer(2, GL_FLOAT, 0, v.astype("float32"))
    glDrawArrays(GL_POINTS, 0, len(v))
    glDisableClientState(GL_VERTEX_ARRAY)
    glFlush()
    glutSwapBuffers()


def halt():
    pass


def keyboard(*_args):
    global window

    if window:
        glutDestroyWindow(window)


def mouse(button, state, _x, _y):
    global angle, delta_angle, move_x, move_y, move_length, halted

    match button:
        case int(GLUT_LEFT_BUTTON):
            angle = angle + delta_angle

        case int(GLUT_RIGHT_BUTTON):
            angle = angle - delta_angle

        case int(GLUT_MIDDLE_BUTTON):
            if state == int(GLUT_DOWN):
                if halted:
                    glutIdleFunc(display)
                    halted = 0
                else:
                    glutIdleFunc(halt)
                    halted = 1

    move_x = move_length * cos(angle)
    move_y = move_length * sin(angle)


def setup_viewport():
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    glOrtho(0.0, 1.0, 0.0, 1.0, 0.0, 1.0)


def reshape(w, h):
    glViewport(0, 0, w, h)
    setup_viewport()


def main():
    global window

    glutInit(sys.argv)
    glutInitDisplayMode(int(GLUT_DOUBLE) | int(GLUT_RGB))
    glutInitWindowSize(300, 300)
    window = glutCreateWindow(b"Dots")

    # Initialize OpenGL state
    glClearColor(0.0, 0.0, 0.0, 0.0)
    setup_viewport()

    # Set callbacks
    glutReshapeFunc(reshape)
    glutDisplayFunc(display)
    glutIdleFunc(display)
    glutMouseFunc(mouse)
    glutKeyboardFunc(keyboard)

    glutMainLoop()


if __name__ == "__main__":
    print("Use the mouse buttons to control some of the dots.")
    print("Hit close button to quit.")
    main()
