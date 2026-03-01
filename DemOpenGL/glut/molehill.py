#!/usr/bin/env python

from OpenGL import GL as gl
from OpenGL import GLU as glu
from OpenGL import GLUT as glut
import numpy
import sys


def display(context, the_list):
    glut.glutSetWindow(context)
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
    gl.glEnable(gl.GL_LIGHT0)
    gl.glEnable(gl.GL_LIGHTING)
    gl.glDisable(gl.GL_CULL_FACE)
    gl.glEnable(gl.GL_DEPTH_TEST)
    gl.glCallList(the_list)
    glut.glutSwapBuffers()


def main():
    mat_red_diffuse = numpy.array((0.7, 0.0, 0.1, 1.0), "f")
    mat_green_diffuse = numpy.array((0.0, 0.7, 0.1, 1.0), "f")
    mat_blue_diffuse = numpy.array((0.0, 0.1, 0.7, 1.0), "f")
    mat_yellow_diffuse = numpy.array((0.7, 0.8, 0.1, 1.0), "f")
    mat_specular = numpy.array((1.0, 1.0, 1.0, 1.0), "f")
    mat_shininess = 100.0
    knots = (0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0)

    glut.glutInit(sys.argv)
    glut.glutInitDisplayMode(
        int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
    )
    context = glut.glutCreateWindow("Molehill")
    glut.glutSetWindow(context)

    nurb = glu.gluNewNurbsRenderer()
    # get a really good sampling
    glu.gluNurbsProperty(nurb, glu.GLU_SAMPLING_TOLERANCE, 5.0)
    glu.gluNurbsProperty(nurb, glu.GLU_DISPLAY_MODE, glu.GLU_FILL)
    # gluNurbsProperty(nurb, GLU_DISPLAY_MODE, GLU_OUTLINE_POLYGON)

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
            pts1[u].append([2.0 * u, 2.0 * v, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                pts1[u][v][2] = 6.0

            # Green.
            pts2[u].append([2.0 * u - 6.0, 2.0 * v - 6.0, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                if u == 1 and v == 1:
                    # Pull hard on single middle square.
                    pts2[u][v][2] = 15.0
                else:
                    # Push down on other middle squares.
                    pts2[u][v][2] = -2.0

            # Blue.
            pts3[u].append([2.0 * u - 6.0, 2.0 * v, 0.0])
            if (u == 1 or u == 2) and (v == 1 or v == 2):
                if u == 1 and v == 2:
                    # Pull up on single middple square.
                    pts3[u][v][2] = 11.0
                else:
                    # Pull up slightly on other middle squares.
                    pts3[u][v][2] = 2.0

            # Yellow.
            pts4[u].append([2.0 * u, 2.0 * v - 6.0, 0.0])
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

    gl.glMatrixMode(gl.GL_PROJECTION)
    glu.gluPerspective(55.0, 1.0, 2.0, 24.0)
    gl.glMatrixMode(gl.GL_MODELVIEW)
    gl.glTranslatef(0.0, 0.0, -15.0)
    gl.glRotatef(330.0, 1.0, 0.0, 0.0)

    the_list = gl.glGenLists(1)
    gl.glNewList(the_list, gl.GL_COMPILE)

    gl.glEnable(gl.GL_AUTO_NORMAL)
    gl.glEnable(gl.GL_NORMALIZE)
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_SPECULAR, mat_specular)
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_SHININESS, mat_shininess)

    # Render red hill.
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, mat_red_diffuse)
    glu.gluBeginSurface(nurb)
    glu.gluNurbsSurface(nurb, knots, knots, pts1, gl.GL_MAP2_VERTEX_3)
    glu.gluEndSurface(nurb)

    # Render green hill.
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, mat_green_diffuse)
    glu.gluBeginSurface(nurb)
    glu.gluNurbsSurface(nurb, knots, knots, pts2, gl.GL_MAP2_VERTEX_3)
    glu.gluEndSurface(nurb)

    # Render blue hill.
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, mat_blue_diffuse)
    glu.gluBeginSurface(nurb)
    glu.gluNurbsSurface(nurb, knots, knots, pts3, gl.GL_MAP2_VERTEX_3)
    glu.gluEndSurface(nurb)

    # Render yellow hill.
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_DIFFUSE, mat_yellow_diffuse)
    glu.gluBeginSurface(nurb)
    glu.gluNurbsSurface(nurb, knots, knots, pts4, gl.GL_MAP2_VERTEX_3)
    glu.gluEndSurface(nurb)
    gl.glEndList()

    glut.glutDisplayFunc(lambda: display(context, the_list))

    glut.glutMainLoop()


if __name__ == "__main__":

    main()
