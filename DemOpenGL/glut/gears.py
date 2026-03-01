#!/usr/bin/env python3

# * 3-D gear wheels.  This program is in the public domain.
# * Brian Paul
# * Conversion to GLUT by Mark J. Kilgard
# conversion to Python using PyOpenGL with frame rates ala gl.glxgears
# Peter Barth

from OpenGL import GL as gl
from OpenGL import GLUT as glut
import sys
import time
from math import sin, cos, sqrt, pi


def gear(inner_radius, outer_radius, width, teeth, tooth_depth):
    r0 = inner_radius
    r1 = outer_radius - tooth_depth / 2.0
    r2 = outer_radius + tooth_depth / 2.0
    da = 2.0 * pi / teeth / 4.0

    gl.glShadeModel(gl.GL_FLAT)
    gl.glNormal3f(0.0, 0.0, 1.0)

    # draw front face
    gl.glBegin(gl.GL_QUAD_STRIP)
    for i in range(teeth + 1):
        angle = i * 2.0 * pi / teeth
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), width * 0.5)
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), width * 0.5)
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), width * 0.5)
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), width * 0.5)
    gl.glEnd()

    # draw front sides of teeth
    gl.glBegin(gl.GL_QUADS)
    da = 2.0 * pi / teeth / 4.0
    for i in range(teeth):
        angle = i * 2.0 * pi / teeth
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), width * 0.5)
        gl.glVertex3f(r2 * cos(angle + da), r2 * sin(angle + da), width * 0.5)
        gl.glVertex3f(r2 * cos(angle + 2 * da), r2 * sin(angle + 2 * da), width * 0.5)
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), width * 0.5)
    gl.glEnd()

    gl.glNormal3f(0.0, 0.0, -1.0)

    # draw back face
    gl.glBegin(gl.GL_QUAD_STRIP)
    for i in range(teeth + 1):
        angle = i * 2.0 * pi / teeth
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), -width * 0.5)
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), -width * 0.5)
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), -width * 0.5)
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), -width * 0.5)
    gl.glEnd()

    # draw back sides of teeth
    gl.glBegin(gl.GL_QUADS)
    da = 2.0 * pi / teeth / 4.0
    for i in range(teeth):
        angle = i * 2.0 * pi / teeth
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), -width * 0.5)
        gl.glVertex3f(r2 * cos(angle + 2 * da), r2 * sin(angle + 2 * da), -width * 0.5)
        gl.glVertex3f(r2 * cos(angle + da), r2 * sin(angle + da), -width * 0.5)
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), -width * 0.5)
    gl.glEnd()

    # draw outward faces of teeth
    gl.glBegin(gl.GL_QUAD_STRIP)
    for i in range(teeth):
        angle = i * 2.0 * pi / teeth
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), width * 0.5)
        gl.glVertex3f(r1 * cos(angle), r1 * sin(angle), -width * 0.5)
        u = r2 * cos(angle + da) - r1 * cos(angle)
        v = r2 * sin(angle + da) - r1 * sin(angle)
        length = sqrt(u * u + v * v)
        u = u / length
        v = v / length
        gl.glNormal3f(v, -u, 0.0)
        gl.glVertex3f(r2 * cos(angle + da), r2 * sin(angle + da), width * 0.5)
        gl.glVertex3f(r2 * cos(angle + da), r2 * sin(angle + da), -width * 0.5)
        gl.glNormal3f(cos(angle), sin(angle), 0.0)
        gl.glVertex3f(r2 * cos(angle + 2 * da), r2 * sin(angle + 2 * da), width * 0.5)
        gl.glVertex3f(r2 * cos(angle + 2 * da), r2 * sin(angle + 2 * da), -width * 0.5)
        u = r1 * cos(angle + 3 * da) - r2 * cos(angle + 2 * da)
        v = r1 * sin(angle + 3 * da) - r2 * sin(angle + 2 * da)
        gl.glNormal3f(v, -u, 0.0)
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), width * 0.5)
        gl.glVertex3f(r1 * cos(angle + 3 * da), r1 * sin(angle + 3 * da), -width * 0.5)
        gl.glNormal3f(cos(angle), sin(angle), 0.0)

    gl.glVertex3f(r1 * cos(0), r1 * sin(0), width * 0.5)
    gl.glVertex3f(r1 * cos(0), r1 * sin(0), -width * 0.5)

    gl.glEnd()

    gl.glShadeModel(gl.GL_SMOOTH)

    # draw inside radius cylinder
    gl.glBegin(gl.GL_QUAD_STRIP)
    for i in range(teeth + 1):
        angle = i * 2.0 * pi / teeth
        gl.glNormal3f(-cos(angle), -sin(angle), 0.0)
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), -width * 0.5)
        gl.glVertex3f(r0 * cos(angle), r0 * sin(angle), width * 0.5)
    gl.glEnd()


(view_rot_x, view_rot_y, view_rot_z) = (20.0, 30.0, 0.0)
(gear_1, gear_2, gear_3) = (0, 0, 0)
angle = 0.0


t_start = t0 = time.time()
frames = 0
rotation_rate = 1.01


def frame_rate():
    global t0, frames
    t = time.time()
    frames += 1
    if t - t0 >= 5.0:
        seconds = t - t0
        fps = frames / seconds
        print("%.0f frames in %3.1f seconds = %6.3f FPS" % (frames, seconds, fps))
        t0 = t
        frames = 0


def draw():
    rotation_rate = (time.time() - t_start) * 1.05
    angle = (2 * pi) * ((time.time() - t_start) * rotation_rate)  # * rotationRate
    gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))

    gl.glPushMatrix()
    gl.glRotatef(view_rot_x, 1.0, 0.0, 0.0)
    gl.glRotatef(view_rot_y, 0.0, 1.0, 0.0)
    gl.glRotatef(view_rot_z, 0.0, 0.0, 1.0)

    gl.glPushMatrix()
    gl.glTranslatef(-3.0, -2.0, 0.0)
    gl.glRotatef(angle, 0.0, 0.0, 1.0)
    gl.glCallList(gear_1)
    gl.glPopMatrix()

    gl.glPushMatrix()
    gl.glTranslatef(3.1, -2.0, 0.0)
    gl.glRotatef(-2.0 * angle - 9.0, 0.0, 0.0, 1.0)
    gl.glCallList(gear_2)
    gl.glPopMatrix()

    gl.glPushMatrix()
    gl.glTranslatef(-3.1, 4.2, 0.0)
    gl.glRotatef(-2.0 * angle - 25.0, 0.0, 0.0, 1.0)
    gl.glCallList(gear_3)
    gl.glPopMatrix()

    gl.glPopMatrix()

    glut.glutSwapBuffers()

    frame_rate()


def idle():
    global angle
    angle += 2.0
    glut.glutPostRedisplay()


# change view angle, exit upon ESC
def key(k, x, y):
    global view_rot_z

    if k == "z":
        view_rot_z += 5.0
    elif k == "Z":
        view_rot_z -= 5.0
    elif ord(k) == 27:  # Escape
        sys.exit(0)
    else:
        return
    glut.glutPostRedisplay()


# change view angle
def special(k, x, y):
    global view_rot_x, view_rot_y, view_rot_z

    match int(k):
        case int(glut.GLUT_KEY_UP):
            view_rot_x += 5.0
        case int(glut.GLUT_KEY_DOWN):
            view_rot_x -= 5.0
        case int(glut.GLUT_KEY_LEFT):
            view_rot_y += 5.0
        case int(glut.GLUT_KEY_RIGHT):
            view_rot_y -= 5.0
        case _:
            return
    glut.glutPostRedisplay()


# new window size or exposure
def reshape(width, height):
    h = float(height) / float(width)
    gl.glViewport(0, 0, width, height)
    gl.glMatrixMode(gl.GL_PROJECTION)
    gl.glLoadIdentity()
    gl.glFrustum(-1.0, 1.0, -h, h, 5.0, 60.0)
    gl.glMatrixMode(gl.GL_MODELVIEW)
    gl.glLoadIdentity()
    gl.glTranslatef(0.0, 0.0, -40.0)


def init():
    global gear_1, gear_2, gear_3

    pos = (5.0, 5.0, 10.0, 0.0)
    red = (0.8, 0.1, 0.0, 1.0)
    green = (0.0, 0.8, 0.2, 1.0)
    blue = (0.2, 0.2, 1.0, 1.0)

    gl.glLightfv(gl.GL_LIGHT0, gl.GL_POSITION, pos)
    gl.glEnable(gl.GL_CULL_FACE)
    gl.glEnable(gl.GL_LIGHTING)
    gl.glEnable(gl.GL_LIGHT0)
    gl.glEnable(gl.GL_DEPTH_TEST)

    # make the gears
    gear_1 = gl.glGenLists(1)
    gl.glNewList(gear_1, gl.GL_COMPILE)
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_AMBIENT_AND_DIFFUSE, red)
    gear(1.0, 4.0, 1.0, 20, 0.7)
    gl.glEndList()

    gear_2 = gl.glGenLists(1)
    gl.glNewList(gear_2, gl.GL_COMPILE)
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_AMBIENT_AND_DIFFUSE, green)
    gear(0.5, 2.0, 2.0, 10, 0.7)
    gl.glEndList()

    gear_3 = gl.glGenLists(1)
    gl.glNewList(gear_3, gl.GL_COMPILE)
    gl.glMaterialfv(gl.GL_FRONT, gl.GL_AMBIENT_AND_DIFFUSE, blue)
    gear(1.3, 2.0, 0.5, 10, 0.7)
    gl.glEndList()

    gl.glEnable(gl.GL_NORMALIZE)


def visible(vis):
    if vis == glut.GLUT_VISIBLE:
        glut.glutIdleFunc(idle)
    else:
        glut.glutIdleFunc(None)


def main():
    glut.glutInit(sys.argv)
    glut.glutInitDisplayMode(
        int(glut.GLUT_RGB) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
    )

    glut.glutInitWindowPosition(0, 0)
    glut.glutInitWindowSize(300, 300)
    glut.glutCreateWindow("PyGears")
    init()

    glut.glutDisplayFunc(draw)
    glut.glutReshapeFunc(reshape)
    glut.glutKeyboardFunc(key)
    glut.glutSpecialFunc(special)
    glut.glutVisibilityFunc(visible)

    if "-info" in sys.argv:
        print("gl.GL_RENDERER   = ", gl.glGetString(gl.GL_RENDERER))
        print("gl.GL_VERSION    = ", gl.glGetString(gl.GL_VERSION))
        print("gl.GL_VENDOR     = ", gl.glGetString(gl.GL_VENDOR))
        print("gl.GL_EXTENSIONS = ", gl.glGetString(gl.GL_EXTENSIONS))

    glut.glutMainLoop()


if __name__ == "__main__":
    main()
