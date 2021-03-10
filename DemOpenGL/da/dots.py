#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
from OpenGL.GL import *
from OpenGL.GLUT import *

from numpy import cos, sin, greater, choose, transpose, concatenate, \
    array
from numpy.random import random, randint


MY_LIST = 1
NUMDOTS = 500
NUMDOTS2 = 600
MAX_AGE = 13

x = random(NUMDOTS) * 2-1
y = random(NUMDOTS) * 2-1
age = randint(0, MAX_AGE, (NUMDOTS,))
angle = 0           # in radians
delta_angle = .2    # in radians
move_length = .005  # 1.0 = screen width
move_x = move_length * cos(angle)
move_y = move_length * sin(angle)
halted = 0


def display(*args):
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
    age = choose(which, (age, 0))
    x = choose(greater(x, 1.0), (x, x-1.0))  # very cool - wraparound
    y = choose(greater(y, 1.0), (y, y-1.0))
    x2 = random(NUMDOTS2)
    y2 = random(NUMDOTS2)
    v = concatenate((transpose(array([x, y])), transpose(array([x-.005, y+.005])),
                     transpose(array([x+.005, y-.005])), transpose(array([x2, y2]))))
    glVertexPointerd(v)
    glEnableClientState(GL_VERTEX_ARRAY)
    glDrawArrays(GL_POINTS, 0, len(v))
    glDisableClientState(GL_VERTEX_ARRAY)
    glFlush()
    glutSwapBuffers()


def halt():
    pass


def keyboard(*args):
    glutDestroyWindow(window)


def mouse(button, state, x, y):
    global angle, delta_angle, move_x, move_y, move_length, halted
    if button == GLUT_LEFT_BUTTON:
        angle = angle + delta_angle
    elif button == GLUT_RIGHT_BUTTON:
        angle = angle - delta_angle
    elif button == GLUT_MIDDLE_BUTTON and state == GLUT_DOWN:
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
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB)
    glutInitWindowSize(300, 300)
    glutCreateWindow('Dots')
    setup_viewport()
    glutReshapeFunc(reshape)
    glutDisplayFunc(display)
    glutIdleFunc(display)
    glutMouseFunc(mouse)
    glutKeyboardFunc(keyboard)
    glutMainLoop()


if __name__ == '__main__':
    print('Use the mouse buttons to control some of the dots.')
    print('Hit close button to quit.')
    main()
