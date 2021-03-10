#!/usr/bin/env python3

from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GLUT import *
import numpy
import sys

THE_LIST = None


def display():
    glutSetWindow(context)
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glEnable(GL_LIGHT0)
    glEnable(GL_LIGHTING)
    glDisable(GL_CULL_FACE)
    glEnable(GL_DEPTH_TEST)
    glCallList(THE_LIST)
    glutSwapBuffers()


def main():
    mat_red_diffuse = numpy.array((0.7, 0.0, 0.1, 1.0), 'f')
    mat_green_diffuse = numpy.array((0.0, 0.7, 0.1, 1.0), 'f')
    mat_blue_diffuse = numpy.array((0.0, 0.1, 0.7, 1.0), 'f')
    mat_yellow_diffuse = numpy.array((0.7, 0.8, 0.1, 1.0), 'f')
    mat_specular = numpy.array((1.0, 1.0, 1.0, 1.0), 'f')
    mat_shininess = 100.0
    knots = (0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0)

    glutInit(sys.argv)
    glutInitDisplayMode(GLUT_RGBA | GLUT_DOUBLE | GLUT_DEPTH)
    global context
    context = glutCreateWindow('Molehill')
    glutSetWindow(context)

    nurb = gluNewNurbsRenderer()
    # get a really good sampling
    gluNurbsProperty(nurb, GLU_SAMPLING_TOLERANCE, 5.0)
    gluNurbsProperty(nurb, GLU_DISPLAY_MODE, GLU_FILL)
    #gluNurbsProperty(nurb, GLU_DISPLAY_MODE, GLU_OUTLINE_POLYGON)

    # Build control points for NURBS mole hills.
    pts1 = []
    pts2 = []
    pts3 = []
    pts4 = []

    for u in range(4):
        pts1.append([])
        pts2.append([])
        pts3.append([])
        pts4.append([])
        for v in range(4):
            # Red.
            pts1[u].append([2.0*u, 2.0*v, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                pts1[u][v][2] = 6.0

            # Green.
            pts2[u].append([2.0*u - 6.0, 2.0*v - 6.0, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                if u == 1 and v == 1:
                    # Pull hard on single middle square.
                    pts2[u][v][2] = 15.0
                else:
                    # Push down on other middle squares.
                    pts2[u][v][2] = -2.0

            # Blue.
            pts3[u].append([2.0*u - 6.0, 2.0*v, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                if u == 1 and v == 2:
                    # Pull up on single middple square.
                    pts3[u][v][2] = 11.0
                else:
                    # Pull up slightly on other middle squares.
                    pts3[u][v][2] = 2.0

            # Yellow.
            pts4[u].append([2.0*u, 2.0*v - 6.0, 0.0])
            if u != 0 and (v == 1 or v == 2):
                if v == 1:
                    # Push down front middle and right squares.
                    pts4[u][v][2] = -2.0
                else:
                    # Pull up back middle and right squares.
                    pts4[u][v][2] = 5.0

    # Stretch up red's far right corner.
    pts1[3][3][2] = 6.0
    # Pull down green's near left corner a little.
    pts2[0][0][2] = -2.0
    # Turn up meeting of four corners.
    pts1[0][0][2] = 1.0
    pts2[3][3][2] = 1.0
    pts3[3][0][2] = 1.0
    pts4[0][3][2] = 1.0

    glMatrixMode(GL_PROJECTION)
    gluPerspective(55.0, 1.0, 2.0, 24.0)
    glMatrixMode(GL_MODELVIEW)
    glTranslatef(0.0, 0.0, -15.0)
    glRotatef(330.0, 1.0, 0.0, 0.0)

    global THE_LIST
    THE_LIST = glGenLists(1)
    glNewList(THE_LIST, GL_COMPILE)

    glEnable(GL_AUTO_NORMAL)
    glEnable(GL_NORMALIZE)
    glMaterialfv(GL_FRONT, GL_SPECULAR, mat_specular)
    glMaterialfv(GL_FRONT, GL_SHININESS, mat_shininess)

    # Render red hill.
    glMaterialfv(GL_FRONT, GL_DIFFUSE, mat_red_diffuse)
    gluBeginSurface(nurb)
    gluNurbsSurface(nurb, knots, knots, pts1, GL_MAP2_VERTEX_3)
    gluEndSurface(nurb)

    # Render green hill.
    glMaterialfv(GL_FRONT, GL_DIFFUSE, mat_green_diffuse)
    gluBeginSurface(nurb)
    gluNurbsSurface(nurb, knots, knots, pts2, GL_MAP2_VERTEX_3)
    gluEndSurface(nurb)

    # Render blue hill.
    glMaterialfv(GL_FRONT, GL_DIFFUSE, mat_blue_diffuse)
    gluBeginSurface(nurb)
    gluNurbsSurface(nurb, knots, knots, pts3, GL_MAP2_VERTEX_3)
    gluEndSurface(nurb)

    # Render yellow hill.
    glMaterialfv(GL_FRONT, GL_DIFFUSE, mat_yellow_diffuse)
    gluBeginSurface(nurb)
    gluNurbsSurface(nurb, knots, knots, pts4, GL_MAP2_VERTEX_3)
    gluEndSurface(nurb)
    glEndList()

    glutDisplayFunc(display)


if __name__ == '__main__':

    main()
    glutMainLoop()
