"""GLUT replacement for the original checker.py demonstration code

Note:
    Has no navigation code ATM.
"""

import time
import sys
from OpenGL.GL import (
    glClear,
    glClearColor,
    glDepthFunc,
    glEnable,
    glGetDoublev,
    glLightfv,
    glLightModelfv,
    glLoadIdentity,
    glMaterialfv,
    glMatrixMode,
    glPopMatrix,
    glPushMatrix,
    glRotate,
    glRotatef,
    glTranslatef,
    GL_AMBIENT,
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
    GL_DEPTH_TEST,
    GL_DIFFUSE,
    GL_FRONT,
    GL_LESS,
    GL_LIGHT0,
    GL_LIGHTING,
    GL_LIGHT_MODEL_AMBIENT,
    GL_MODELVIEW,
    GL_POSITION,
    GL_PROJECTION,
    GL_SHININESS,
    GL_SPECULAR,
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
    glutSolidCone,
    glutSwapBuffers,
    GLUT_DEPTH,
    GLUT_DOUBLE,
    GLUT_RGBA,
)


def draw_cone(position=(0, -1, 0), radius=1, height=2, slices=50, stacks=10):
    glPushMatrix()
    try:
        glTranslatef(*position)
        glRotatef(250, 1, 0, 0)
        glutSolidCone(radius, height, slices, stacks)
    finally:
        glPopMatrix()


def cone_material():
    """Setup material for cone"""
    glMaterialfv(GL_FRONT, GL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
    glMaterialfv(GL_FRONT, GL_DIFFUSE, [0.8, 0.8, 0.8, 1.0])
    glMaterialfv(GL_FRONT, GL_SPECULAR, [1.0, 0.0, 1.0, 1.0])
    glMaterialfv(GL_FRONT, GL_SHININESS, 50.0)


def light():
    """Setup light 0 and enable lighting"""
    glLightfv(GL_LIGHT0, GL_AMBIENT, [0.0, 1.0, 0.0, 1.0])
    glLightfv(GL_LIGHT0, GL_DIFFUSE, [1.0, 1.0, 1.0, 1.0])
    glLightfv(GL_LIGHT0, GL_SPECULAR, [1.0, 1.0, 1.0, 1.0])
    glLightfv(GL_LIGHT0, GL_POSITION, [1.0, 1.0, 1.0, 0.0])
    glLightModelfv(GL_LIGHT_MODEL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
    glEnable(GL_LIGHTING)
    glEnable(GL_LIGHT0)


def depth():
    """Setup depth testing"""
    glDepthFunc(GL_LESS)
    glEnable(GL_DEPTH_TEST)


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
        5,  # eyepoint
        0,
        0,
        0,  # center-of-view
        0,
        1,
        0,  # up-vector
    )
    light()
    depth()
    cone_material()

    rotation()
    draw_cone()
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
    print("You should see a high-resolution cone rotating slowly.")

    glutInit(sys.argv)
    glutInitDisplayMode(int(GLUT_RGBA) | int(GLUT_DOUBLE) | int(GLUT_DEPTH))
    glutCreateWindow("Rotating Cone")
    glutDisplayFunc(display)
    glutIdleFunc(display)
    # note need to do this to properly render faceted geometry
    glutMainLoop()


if __name__ == "__main__":
    main()
