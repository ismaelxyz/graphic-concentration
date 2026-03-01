"""
GlutPlane - A GLUT-based 3D plane simulation with OOP design.

This module demonstrates an object-oriented approach to a 3D OpenGL/GLUT
application showing animated planes flying in formation.
"""

import sys
import enum
from OpenGL.GL import (
    glClear,
    glEnable,
    glDisable,
    glBegin,
    glEnd,
    glVertex3f,
    glColor3f,
    glPushMatrix,
    glPopMatrix,
    glTranslatef,
    glRotatef,
    glScalef,
    glShadeModel,
    glMatrixMode,
    glFrustum,
    glClearDepth,
    glClearColor,
    GL_DEPTH_BUFFER_BIT,
    GL_DEPTH_TEST,
    GL_SMOOTH,
    GL_FLAT,
    GL_POLYGON,
    GL_TRIANGLE_STRIP,
    GL_PROJECTION,
    GL_MODELVIEW,
)
from OpenGL.GLUT import (
    glutInit,
    glutInitWindowPosition,
    glutInitWindowSize,
    glutSetOption,
    glutLeaveMainLoop,
    glutInitDisplayMode,
    glutCreateWindow,
    glutDisplayFunc,
    glutKeyboardFunc,
    glutVisibilityFunc,
    glutIdleFunc,
    glutPostRedisplay,
    glutChangeToMenuEntry,
    glutCreateMenu,
    glutAddMenuEntry,
    glutAttachMenu,
    glutMainLoop,
    glutSwapBuffers,
    GLUT_VISIBLE,
    GLUT_RIGHT_BUTTON,
    GLUT_DOUBLE,
    GLUT_RGB,
    GLUT_DEPTH,
    GLUT_MULTISAMPLE,
    GLUT_ACTION_ON_WINDOW_CLOSE,
    GLUT_ACTION_GLUTMAINLOOP_RETURNS,
)
from math import cos, sin, atan, pi
from random import choice, randint, getrandbits


class Plane:
    """Represents a single plane with its properties and behavior."""

    M_PI = pi
    M_PI_2 = pi / 2.0

    # Color options for planes
    RGB_LIST = [
        (1.0, 0.0, 0.0),  # red
        (1.0, 1.0, 1.0),  # white
        (0.0, 1.0, 0.0),  # green
        (1.0, 0.0, 1.0),  # magenta
        (1.0, 1.0, 0.0),  # yellow
        (0.0, 1.0, 1.0),  # cyan
    ]

    def __init__(self):
        """Initialize a plane with default (inactive) values."""
        self.speed = 0.0
        self.red = 0.0
        self.green = 0.0
        self.blue = 0.0
        self.theta = 0.0
        self.angle = 0.0
        self.x = 0.0
        self.y = 0.0
        self.z = 0.0

    def is_active(self):
        """Check if the plane is currently active (moving)."""
        return self.speed != 0.0

    def activate(self):
        """Activate the plane with random parameters."""
        self.red, self.green, self.blue = choice(self.RGB_LIST)
        self.speed = (float(randint(0, 19)) * 0.001) + 0.02
        if getrandbits(32) & 0x1:
            self.speed *= -1
        self.theta = float(randint(0, 256)) * 0.1111
        self.update_position()

    def deactivate(self):
        """Deactivate the plane."""
        self.speed = 0.0

    def update_position(self):
        """Update the plane's position based on its theta value."""
        self.theta += self.speed
        theta = self.theta
        self.z = -10 + 4 * cos(theta)
        self.x = 5 * sin(2 * theta)
        self.y = sin(theta / 3.4) * 3
        self.angle = (
            ((atan(2.0) + self.M_PI_2) * sin(theta) - self.M_PI_2) * 180 / self.M_PI
        )
        if self.speed < 0.0:
            self.angle += 180.0

    def draw(self):
        """Draw the plane using OpenGL."""
        if not self.is_active():
            return

        glPushMatrix()
        glTranslatef(self.x, self.y, self.z)
        glRotatef(290.0, 1.0, 0.0, 0.0)
        glRotatef(self.angle, 0.0, 0.0, 1.0)
        glScalef(1.0 / 3.0, 1.0 / 4.0, 1.0 / 4.0)
        glTranslatef(0.0, -4.0, -1.5)
        glBegin(GL_TRIANGLE_STRIP)

        # left wing
        glVertex3f(-7.0, 0.0, 2.0)
        glVertex3f(-1.0, 0.0, 3.0)
        red = self.red
        green = self.green
        blue = self.blue
        glColor3f(red, green, blue)
        glVertex3f(-1.0, 7.0, 3.0)

        # left side
        glColor3f(0.6 * red, 0.6 * green, 0.6 * blue)
        glVertex3f(0.0, 0.0, 0.0)
        glVertex3f(0.0, 8.0, 0.0)

        # right side
        glVertex3f(1.0, 0.0, 3.0)
        glVertex3f(1.0, 7.0, 3.0)

        # final tip of right wing
        glColor3f(red, green, blue)
        glVertex3f(7.0, 0.0, 2.0)

        glEnd()
        glPopMatrix()


class PlaneSimulationAction(enum.Enum):
    VOID = 0
    ADD_PLANE = 1
    REMOVE_PLANE = 2
    MOTION_ON = 3
    MOTION_OFF = 4
    QUIT = 5


class PlaneSimulation:
    """Manages the plane simulation including rendering, animation, and input."""

    # Menu constants

    def __init__(self, max_planes=15):
        """Initialize the plane simulation."""
        self.max_planes = max_planes
        self.planes = [Plane() for _ in range(max_planes)]
        self.moving = False

    def get_inactive_plane(self):
        """Find and return the first inactive plane, or None if all are active."""
        for plane in self.planes:
            if not plane.is_active():
                return plane
        return None

    def add_plane(self):
        """Activate a new plane if there's an available slot."""
        plane = self.get_inactive_plane()
        if plane:
            plane.activate()
            if not self.moving:
                glutPostRedisplay()

    def remove_plane(self):
        """Deactivate the most recently added plane."""
        for i in range(self.max_planes - 1, -1, -1):
            if self.planes[i].is_active():
                self.planes[i].deactivate()
                if not self.moving:
                    glutPostRedisplay()
                return

    def tick(self):
        """Update all active planes."""
        for plane in self.planes:
            if plane.is_active():
                plane.update_position()

    def animate(self):
        """Animation callback for GLUT idle function."""
        self.tick()
        glutPostRedisplay()

    def draw_background(self):
        """Draw the background gradient (black to blue)."""
        glDisable(GL_DEPTH_TEST)
        glShadeModel(GL_SMOOTH)
        glBegin(GL_POLYGON)
        glColor3f(0.0, 0.0, 0.0)
        glVertex3f(-20.0, 20.0, -19.0)
        glVertex3f(20.0, 20.0, -19.0)
        glColor3f(0.0, 0.0, 1.0)
        glVertex3f(20.0, -20.0, -19.0)
        glVertex3f(-20.0, -20.0, -19.0)
        glEnd()

    def draw(self):
        """Main display callback - renders the scene."""
        glClear(GL_DEPTH_BUFFER_BIT)

        # Draw background
        self.draw_background()

        # Draw planes
        glEnable(GL_DEPTH_TEST)
        glShadeModel(GL_FLAT)
        for plane in self.planes:
            plane.draw()

        glutSwapBuffers()

    def visible(self, state):
        """Visibility callback for GLUT."""
        if state == GLUT_VISIBLE:
            if self.moving:
                glutIdleFunc(self.animate)
        else:
            if self.moving:
                glutIdleFunc(None)

    def keyboard(self, ch, x, y):
        """Keyboard callback."""
        if ch == " ":
            if not self.moving:
                self.tick()
                glutPostRedisplay()
        elif ch == chr(27):
            sys.exit(0)
        return 0

    def motion_on(self):
        """Enable motion animation."""
        self.moving = True
        glutChangeToMenuEntry(3, "Motion off", PlaneSimulationAction.MOTION_OFF.value)
        glutIdleFunc(self.animate)

    def motion_off(self):
        """Disable motion animation."""
        self.moving = False
        glutChangeToMenuEntry(3, "Motion", PlaneSimulationAction.MOTION_ON.value)
        glutIdleFunc(None)

    def quit(self):
        """Exit the application."""
        glutLeaveMainLoop()

    def menu(self, item):
        """Menu callback handler."""

        match item:
            case PlaneSimulationAction.ADD_PLANE.value:
                self.add_plane()
            case PlaneSimulationAction.REMOVE_PLANE.value:
                self.remove_plane()
            case PlaneSimulationAction.MOTION_ON.value:
                self.motion_on()
            case PlaneSimulationAction.MOTION_OFF.value:
                self.motion_off()
            case PlaneSimulationAction.QUIT.value:
                self.quit()

        return 0

    def init_glut(self, argv=None):
        """Initialize GLUT and create window."""
        if argv is None:
            argv = ["glutplane"]
        glutInit(argv)
        glutInitWindowPosition(112, 84)
        glutInitWindowSize(800, 600)
        glutSetOption(GLUT_ACTION_ON_WINDOW_CLOSE, GLUT_ACTION_GLUTMAINLOOP_RETURNS)
        # use multisampling if available
        glutInitDisplayMode(
            int(GLUT_DOUBLE) | int(GLUT_RGB) | int(GLUT_DEPTH) | int(GLUT_MULTISAMPLE)
        )

        glutCreateWindow("GlutPlane")
        glutDisplayFunc(self.draw)
        glutKeyboardFunc(self.keyboard)
        glutVisibilityFunc(self.visible)

        # Create menu
        glutCreateMenu(self.menu)
        glutAddMenuEntry("Add plane", PlaneSimulationAction.ADD_PLANE.value)
        glutAddMenuEntry("Remove plane", PlaneSimulationAction.REMOVE_PLANE.value)
        glutAddMenuEntry("Motion", PlaneSimulationAction.MOTION_ON.value)
        glutAddMenuEntry("Quit", PlaneSimulationAction.QUIT.value)
        glutAttachMenu(GLUT_RIGHT_BUTTON)

        # Setup OpenGL state
        glClearDepth(1.0)
        glClearColor(0.0, 0.0, 0.0, 0.0)
        glMatrixMode(GL_PROJECTION)
        glFrustum(-1.0, 1.0, -1.0, 1.0, 1.0, 30)
        glMatrixMode(GL_MODELVIEW)

    def run(self):
        """Initialize and run the simulation."""
        self.init_glut()

        # Add three initial random planes
        self.add_plane()
        self.add_plane()
        self.add_plane()

        print("RIGHT-CLICK to display the menu.")
        glutMainLoop()


def main():
    """Main entry point."""
    simulation = PlaneSimulation()
    simulation.run()


if __name__ == "__main__":
    main()
