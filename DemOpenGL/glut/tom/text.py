"""GLUT replacement for the original text.py demonstration code

Note:
    Has no navigation code ATM.
"""

from OpenGL.GL import (
    glCallList,
    glClear,
    glClearColor,
    glEnable,
    glEndList,
    glGenLists,
    glGetDoublev,
    glLightfv,
    glLightModelfv,
    glLoadIdentity,
    glMaterialfv,
    glMatrixMode,
    glNewList,
    glRotate,
    glFrontFace,
    GL_AMBIENT,
    GL_COLOR_BUFFER_BIT,
    GL_CCW,
    GL_COMPILE,
    GL_CULL_FACE,
    GL_DEPTH_BUFFER_BIT,
    GL_DEPTH_TEST,
    GL_DIFFUSE,
    GL_FRONT,
    GL_LIGHT0,
    GL_LIGHTING,
    GL_LIGHT_MODEL_AMBIENT,
    GL_MODELVIEW,
    GL_POSITION,
    GL_PROJECTION,
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
    glutSwapBuffers,
    GLUT_DEPTH,
    GLUT_DOUBLE,
    GLUT_RGBA,
)
import time
import sys
from typing import Optional, cast
from logo import define_logo


# Global variable for text display list
text_display_list: Optional[int] = None


def create_list() -> int:
    """Create display list for the text"""
    # PyOpenGL's type hints/stubs may report `glGenLists` as returning `None`.
    # At runtime it returns an integer list id (0 indicates failure).
    list_id = cast(int, glGenLists(1))
    if list_id == 0:
        raise RuntimeError("glGenLists failed to allocate a display list")

    glNewList(list_id, GL_COMPILE)
    try:
        define_logo()
    finally:
        glEndList()
    return list_id


def light():
    """Setup light 0 and enable lighting"""
    glLightfv(GL_LIGHT0, GL_AMBIENT, [0.0, 1.0, 0.0, 1.0])
    glLightfv(GL_LIGHT0, GL_DIFFUSE, [1.0, 1.0, 1.0, 1.0])
    glLightfv(GL_LIGHT0, GL_SPECULAR, [1.0, 1.0, 1.0, 1.0])
    glLightfv(GL_LIGHT0, GL_POSITION, [1.0, 1.0, 1.0, 0.0])
    glLightModelfv(GL_LIGHT_MODEL_AMBIENT, [0.2, 0.2, 0.2, 1.0])
    glEnable(GL_LIGHTING)
    glEnable(GL_LIGHT0)


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
        30,  # eyepoint
        10,
        0,
        0,  # center-of-view
        0,
        1,
        0,  # up-vector
    )
    light()
    rotation()

    glFrontFace(GL_CCW)
    glEnable(GL_CULL_FACE)  # added by jfp to use with new logo.py
    glEnable(GL_DEPTH_TEST)
    glMaterialfv(GL_FRONT, GL_DIFFUSE, [1.0, 1.0, 0.0, 0.0])

    global text_display_list

    if text_display_list is None:
        text_display_list = create_list()

    list_id = text_display_list
    if list_id is None:
        return
    glCallList(list_id)

    if swap:
        glutSwapBuffers()


def idle():
    glutPostRedisplay()


start_time = time.time()


def rotation(period=10):
    """Do rotation of the scene at given rate"""
    angle = (((time.time() - start_time) % period) / period) * 360
    glRotate(angle, 0, 1, 0)
    return angle


def main():
    print("You should see polygonal text rotating slowly.")

    glutInit(sys.argv)
    glutInitDisplayMode(int(GLUT_RGBA) | int(GLUT_DOUBLE) | int(GLUT_DEPTH))
    glutCreateWindow("Polygonal Geometry Demo")
    glutDisplayFunc(display)
    glutIdleFunc(idle)
    # note need to do this to properly render faceted geometry
    glutMainLoop()


if __name__ == "__main__":
    main()
