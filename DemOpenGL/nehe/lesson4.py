#!/usr/bin/env python3

"""
OpenGL Lesson 4: Rotating Triangle and Quad
A simple OpenGL example demonstrating rotation animations using GLUT.
Converted to object-oriented style for cleaner code.
"""

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.GLU as glu
import sys


class GLWindow:
    """A class to encapsulate GLUT window and OpenGL rendering."""

    ESCAPE = b"\033"

    def __init__(
        self,
        width=640,
        height=480,
        title="Jeff Molofee's GL Code Tutorial ... NeHe '99",
    ):
        self.width = width
        self.height = height
        self.title = title
        # Rotation angles
        self.triangle_angle = 0.0
        self.quad_angle = 0.0

    def init_gl(self):
        """Initialize OpenGL settings."""
        # Clear background to black
        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(self.width) / float(self.height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def resize_scene(self, width, height):
        """Handle window resize."""
        if height == 0:
            height = 1

        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def draw_scene(self):
        """Main rendering function."""
        # Clear screen and depth buffer
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()

        # Move left 1.5 units and into the screen 6.0 units
        gl.glTranslatef(-1.5, 0.0, -6.0)

        # Draw a triangle rotated on the Y axis
        gl.glRotatef(self.triangle_angle, 0.0, 1.0, 0.0)
        gl.glBegin(gl.GL_POLYGON)
        gl.glColor3f(1.0, 0.0, 0.0)  # Red
        gl.glVertex3f(0.0, 1.0, 0.0)  # Top
        gl.glColor3f(0.0, 1.0, 0.0)  # Green
        gl.glVertex3f(1.0, -1.0, 0.0)  # Bottom Right
        gl.glColor3f(0.0, 0.0, 1.0)  # Blue
        gl.glVertex3f(-1.0, -1.0, 0.0)  # Bottom Left
        gl.glEnd()

        # Reset matrix for next object
        gl.glLoadIdentity()

        # Move right 1.5 units and into the screen 6.0 units
        gl.glTranslatef(1.5, 0.0, -6.0)

        # Draw a square rotated on the X axis
        gl.glRotatef(self.quad_angle, 1.0, 0.0, 0.0)
        gl.glColor3f(0.3, 0.5, 1.0)  # Bluish shade
        gl.glBegin(gl.GL_QUADS)
        gl.glVertex3f(-1.0, 1.0, 0.0)  # Top Left
        gl.glVertex3f(1.0, 1.0, 0.0)  # Top Right
        gl.glVertex3f(1.0, -1.0, 0.0)  # Bottom Right
        gl.glVertex3f(-1.0, -1.0, 0.0)  # Bottom Left
        gl.glEnd()

        # Update rotation angles
        self.triangle_angle += 1.0
        self.quad_angle -= 1.0

        # Swap buffers for double buffering
        glut.glutSwapBuffers()

    def key_pressed(self, key, x, y):
        """Handle keyboard input."""
        if key == self.ESCAPE:
            glut.glutDestroyWindow(self.window)

    def run(self):
        """Initialize and run the GLUT application."""
        glut.glutInit(sys.argv)

        # Display mode: Double buffer, RGBA, Alpha, Depth
        glut.glutInitDisplayMode(
            int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
        )

        glut.glutInitWindowSize(self.width, self.height)
        glut.glutInitWindowPosition(0, 0)

        self.window = glut.glutCreateWindow(self.title)

        # Register callbacks
        glut.glutDisplayFunc(self.draw_scene)
        glut.glutIdleFunc(self.draw_scene)
        glut.glutReshapeFunc(self.resize_scene)
        glut.glutKeyboardFunc(self.key_pressed)

        # Initialize OpenGL
        self.init_gl()

        print("Hit ESC key to quit.")

        glut.glutMainLoop()


def main():
    app = GLWindow()
    app.run()


if __name__ == "__main__":
    main()
