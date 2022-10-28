#!/usr/bin/env python3
'''
    fog.c from the Redbook examples.  
    Converted to Python by Jason L. Petrone 7/00


    This program draws 5 red spheres, each at a different 
    z distance from the eye, in different types of fog.  
    Pressing the f key chooses between 3 types of 
    fog:  exponential, exponential squared, and linear.  
    In this program, there is a fixed density value, as well 
    as fixed start and end values for the linear fog.
'''


import sys
from OpenGL.GLUT import *
from OpenGL.GL import *
window = None

def init():
    """
        Initialize depth buffer, fog, light source,
        material property, and lighting model.
    """
    position = [0.5, 0.5, 3.0, 0.0]
    glEnable(GL_DEPTH_TEST)
    glLightfv(GL_LIGHT0, GL_POSITION, position)
    glEnable(GL_LIGHTING)
    glEnable(GL_LIGHT0)
    mat = [0.1745, 0.01175, 0.01175, 1.0]
    glMaterialfv(GL_FRONT, GL_AMBIENT, mat)
    mat[0] = 0.61424
    mat[1] = 0.04136
    mat[2] = 0.04136
    glMaterialfv(GL_FRONT, GL_DIFFUSE, mat)
    mat[0] = 0.727811
    mat[1] = 0.626959
    mat[2] = 0.626959
    glMaterialfv(GL_FRONT, GL_SPECULAR, mat)
    glMaterialf(GL_FRONT, GL_SHININESS, 0.6*128.0)

    glEnable(GL_FOG)
    fogColor = [0.5, 0.5, 0.5, 1.0]

    global fogMode
    fogMode = GL_EXP
    glFogi(GL_FOG_MODE, fogMode)
    glFogfv(GL_FOG_COLOR, fogColor)
    glFogf(GL_FOG_DENSITY, 0.35)
    glHint(GL_FOG_HINT, GL_DONT_CARE)
    glFogf(GL_FOG_START, 1.0)
    glFogf(GL_FOG_END, 5.0)
    glClearColor(0.5, 0.5, 0.5, 1.0)


def renderSphere(x, y, z):
    glPushMatrix()
    glTranslatef(x, y, z)
    glutSolidSphere(0.4, 16, 16)
    glPopMatrix()


def display():
    """ display() draws 5 spheres at different z positions. """
    print('Redisplay')
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    renderSphere(-2., -0.5, -1.0)
    renderSphere(-1., -0.5, -2.0)
    renderSphere(0., -0.5, -3.0)
    renderSphere(1., -0.5, -4.0)
    renderSphere(2., -0.5, -5.0)
    glFlush()


def reshape(w, h):
    glViewport(0, 0, w, h)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    if (w <= h):
        glOrtho(-2.5, 2.5, -2.5*h/w, 2.5*h/w, -10.0, 10.0)
    else:
        glOrtho(-2.5*w/h, 2.5*w/h, -2.5, 2.5, -10.0, 10.0)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()


def keyboard(key, x, y):
    global fogMode
    if key in [b'f', b'F']:

        if fogMode == GL_EXP:
            fogMode = GL_EXP2
            print('Fog mode is GL_EXP2')

        elif fogMode == GL_EXP2:
            fogMode = GL_LINEAR
            print('Fog mode is GL_LINEAR')

        elif fogMode == GL_LINEAR:
            fogMode = GL_EXP
            print('Fog mode is GL_EXP')

        glFogi(GL_FOG_MODE, fogMode)
        glutPostRedisplay()

    elif ord(key) == 27:
        glutDestroyWindow(window)


def main():
    """
        Main Loop
        Open window with initial window size, title bar,
        RGBA display mode, depth buffer, and handle input events.
    """
    global window
    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB | GLUT_DEPTH)
    glutInitWindowSize(500, 500)
    window = glutCreateWindow('Fog')
    init()
    glutReshapeFunc(reshape)
    glutKeyboardFunc(keyboard)
    glutDisplayFunc(display)
    glutMainLoop()


if __name__ == '__main__':
    main()
