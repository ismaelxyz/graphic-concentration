"""GLUT replacement for the original checker.py demonstration code

Note:
    Has no navigation code ATM.
"""

from OpenGL.GL import (
    glColor3fv,
    glDisable,
    glEnable,
    glGetDoublev,
    glLightfv,
    glLoadIdentity,
    glMatrixMode,
    glPopMatrix,
    glPushMatrix,
    glRectf,
    glRotate,
    glTranslatef,
    glClear,
    glClearColor,
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
    GL_DEPTH_TEST,
    GL_DIFFUSE,
    GL_LIGHT0,
    GL_LIGHTING,
    GL_MODELVIEW,
    GL_POSITION,
    GL_PROJECTION,
    GL_VIEWPORT,
)
from OpenGL.GLU import gluLookAt, gluPerspective
from OpenGL.GLUT import (
    glutCreateWindow,
    glutDisplayFunc,
    glutIdleFunc,
    glutInit,
    glutInitDisplayMode,
    glutMainLoop,
    glutPostRedisplay,
    glutSolidSphere,
    glutSwapBuffers,
    GLUT_DEPTH,
    GLUT_DOUBLE,
    GLUT_RGBA,
)
import time
import sys


def draw_checker_board(N=5, white=(1, 1, 1), black=(0, 0, 0)):
    """Draw an 2N*2N checkerboard with given colours"""
    glDisable(GL_LIGHTING)
    try:
        for x in range(-N, N):
            for y in range(-N, N):
                if (x + y) % 2 == 0:
                    glColor3fv(white)
                else:
                    glColor3fv(black)
                glRectf(x, y, x + 1, y + 1)
    finally:
        glEnable(GL_LIGHTING)


def draw_sphere(center=(0, 0, 1), radius=1.0, sides=20):
    glPushMatrix()
    try:
        glTranslatef(*center)
        glutSolidSphere(radius, sides, sides)
    finally:
        glPopMatrix()


def display(swap=1, clear=1):
    """Callback function for displaying the scene

    This defines a unit-square environment in which to draw,
    i.e. width is one drawing unit, as is height
    """
    if clear:
        glClearColor(0.5, 0.5, 0.5, 0)
        glClear(int(GL_COLOR_BUFFER_BIT) | int(GL_DEPTH_BUFFER_BIT))

    # establish the projection matrix (perspective)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    _x, _y, width, height = glGetDoublev(GL_VIEWPORT)
    gluPerspective(
        45,  # field of view in degrees
        width / float(height or 1),  # aspect ratio
        0.25,  # near clipping plane
        200,  # far clipping plane
    )

    # and then the model view matrix
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    gluLookAt(
        0,
        1,
        20,  # eyepoint
        0,
        0,
        0,  # center-of-view
        0,
        1,
        0,  # up-vector
    )
    glLightfv(GL_LIGHT0, GL_DIFFUSE, (0.8, 0.8, 0.3))
    glLightfv(GL_LIGHT0, GL_POSITION, (1, 1, 3, 0))
    glEnable(GL_LIGHT0)

    rotation()
    draw_checker_board()
    draw_sphere()
    if swap:
        glutSwapBuffers()


def idle():
    glutPostRedisplay()


starttime = time.time()


def rotation(period=10):
    """Do rotation of the scene at given rate"""
    angle = (((time.time() - starttime) % period) / period) * 360
    glRotate(angle, 0, 1, 0)
    return angle


def main():
    print("You should see a sphere+checker-board rotating about the origin.")

    glutInit(sys.argv)
    glutInitDisplayMode(int(GLUT_RGBA) | int(GLUT_DOUBLE) | int(GLUT_DEPTH))
    glutCreateWindow("Rotating Checkerboard")
    glutDisplayFunc(display)
    glutIdleFunc(display)
    # note need to do this to properly render faceted geometry
    glEnable(GL_DEPTH_TEST)
    glutMainLoop()


if __name__ == "__main__":
    main()
