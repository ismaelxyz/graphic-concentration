#!/usr/bin/env python3

# Now, I assume you've read the prior tutorial notes and know the deal here.  The one major, new requirement
# is to have a working version of PIL (Python Image Library) on your machine.

from __future__ import annotations

import sys
from math import pi
from typing import Any, Callable

import OpenGL.GL as gl
import OpenGL.GLUT as glut
import OpenGL.GLU as glu
from OpenGL.GL.ARB.multitexture import GL_TEXTURE0_ARB, GL_TEXTURE1_ARB
from PIL import Image


class Lesson6Multi:
    """OpenGL lesson 6 - Multi-textured cube rendering."""

    ESCAPE = b"\033"

    def __init__(self):
        """Initialize the lesson with default values."""
        self.window = 0
        # Rotation for cube
        self.rot = 0.0
        self.texture = 0

        # Multi-texture function pointers (overridden in init_gl)
        def _uninitialized(*_args: Any, **_kwargs: Any) -> None:
            raise RuntimeError("Multi-texture not initialized (call init_gl first)")

        def _uninitialized_init() -> bool:
            return True

        self.gl_multi_tex_coord2f: Callable[[Any, float, float], None] = _uninitialized
        self.gl_active_texture: Callable[[Any], None] = _uninitialized

        self._multitexture_ok = False

        # Constants (never None; will be overridden in init_gl if needed)
        self.gl_texture_0: int = int(GL_TEXTURE0_ARB)
        self.gl_texture_1: int = int(GL_TEXTURE1_ARB)

        self.glInitMultitextureARB: Callable[[], bool] = _uninitialized_init

        # Conversion factor for degrees to radians
        self.deg_rad = pi / 180.0

    def load_texture(self, name):
        """Load a texture from file.

        Args:
            name: Path to the image file

        Returns:
            Texture ID
        """
        image = Image.open(name)

        ix = image.size[0]
        iy = image.size[1]
        image = image.tobytes("raw", "RGBX", 0, -1)

        # Create Texture
        texture_id = gl.glGenTextures(1)
        gl.glBindTexture(gl.GL_TEXTURE_2D, texture_id)  # 2d texture (x and y size)

        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D, 0, 3, ix, iy, 0, gl.GL_RGBA, gl.GL_UNSIGNED_BYTE, image
        )
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S, gl.GL_CLAMP)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T, gl.GL_CLAMP)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_S, gl.GL_REPEAT)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_WRAP_T, gl.GL_REPEAT)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_NEAREST)
        gl.glTexParameterf(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_NEAREST)
        gl.glTexEnvf(gl.GL_TEXTURE_ENV, gl.GL_TEXTURE_ENV_MODE, gl.GL_DECAL)

        return texture_id

    def init_gl(self, width, height):
        """A general OpenGL initialization function.

        Sets all of the initial parameters.
        We call this right after our OpenGL window is created.
        """
        # Try to get multitexture functions from OpenGL 1.3 or extensions
        using_builtin = False
        try:
            from OpenGL.GL import glMultiTexCoord2f, glActiveTexture

            self.gl_multi_tex_coord2f = glMultiTexCoord2f
            self.gl_active_texture = glActiveTexture
            self.gl_texture_0 = int(gl.GL_TEXTURE0)
            self.gl_texture_1 = int(gl.GL_TEXTURE1)
            using_builtin = True
            self._multitexture_ok = True
        except (ImportError, AttributeError):
            pass

        # Try to get extension versions if not available
        if not using_builtin:
            try:
                from OpenGL.GL.ARB.multitexture import (
                    glMultiTexCoord2fARB,
                    glActiveTextureARB,
                )

                self.gl_multi_tex_coord2f = glMultiTexCoord2fARB
                self.gl_active_texture = glActiveTextureARB
                self.gl_texture_0 = int(GL_TEXTURE0_ARB)
                self.gl_texture_1 = int(GL_TEXTURE1_ARB)
                self._multitexture_ok = True
            except ImportError:
                pass

        print("Checking for extension support")

        if not self._multitexture_ok:
            print(
                "No OpenGL v1.3 built-in multi-texture support, checking for extension"
            )
            print("No GL_ARB_multitexture support, sorry, cannot run this demo!")
            glut.glutDestroyWindow(self.window)
            raise SystemExit(1)
        else:
            if using_builtin:
                print("Using OpenGL v1.3 built-in multi-texture support")
            else:
                print("Using GL_ARB_multitexture extension")

        try:
            from OpenGL.GL.ARB.multitexture import glInitMultitextureARB

            self.glInitMultitextureARB = glInitMultitextureARB
            if not self.glInitMultitextureARB():
                print("Help!  No GL_ARB_multitexture")
                glut.glutDestroyWindow(self.window)
                raise SystemExit(1)
        except (ImportError, NameError):
            # don't need to init a built-in (or an extension any more, for
            # that matter)
            pass

        self.gl_active_texture(self.gl_texture_0)
        self.load_texture("Wall.bmp")
        gl.glEnable(gl.GL_TEXTURE_2D)

        self.gl_active_texture(self.gl_texture_1)
        self.load_texture("NeHe.bmp")
        gl.glEnable(gl.GL_TEXTURE_2D)

        gl.glTexEnvi(gl.GL_TEXTURE_ENV, gl.GL_TEXTURE_ENV_MODE, gl.GL_BLEND)

        # This Will Clear The Background Color To Black
        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)  # Enables Clearing Of The Depth Buffer
        gl.glDepthFunc(gl.GL_LESS)  # The Type Of Depth Test To Do
        gl.glEnable(gl.GL_DEPTH_TEST)  # Enables Depth Testing
        gl.glShadeModel(gl.GL_SMOOTH)  # Enables Smooth Color Shading

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()  # Reset The Projection Matrix
        # Calculate The Aspect Ratio Of The Window
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)

        gl.glMatrixMode(gl.GL_MODELVIEW)

    def resize_scene(self, width, height):
        """The function called when our window is resized.

        This shouldn't happen if you enable fullscreen.
        """
        if height == 0:  # Prevent A Divide By Zero If The Window Is Too Small
            height = 1

        # Reset The Current Viewport And Perspective Transformation
        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def draw_scene(self):
        """The main drawing function."""
        # Clear The Screen And The Depth Buffer
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()  # Reset The View
        gl.glTranslatef(0.0, 0.0, -5.0)  # Move Into The Screen

        gl.glRotatef(self.rot, 1.0, 0.0, 0.0)  # Rotate The Cube On It's X Axis
        gl.glRotatef(self.rot, 0.0, 1.0, 0.0)  # Rotate The Cube On It's Y Axis
        gl.glRotatef(self.rot, 0.0, 0.0, 1.0)  # Rotate The Cube On It's Z Axis

        p = self.rot * self.deg_rad
        p = p - int(p / (2 * pi)) * 2 * pi  # Normalize to 0-2pi
        p = (abs(p - pi) / pi) ** 2  # Mirror the value for smooth blending
        gl.glTexEnvfv(gl.GL_TEXTURE_ENV, gl.GL_TEXTURE_ENV_COLOR, (p, p, p, 1))

        gl.glBegin(gl.GL_QUADS)  # Start Drawing The Cube

        # Front Face (note that the texture's corners have to match the quad's corners)
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Left Of The Texture and Quad

        # Back Face
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Left Of The Texture and Quad

        # Top Face
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Bottom Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Bottom Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad

        # Bottom Face
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Top Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Top Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad

        # Right face
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)  # Bottom Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)  # Top Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)  # Top Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)  # Bottom Left Of The Texture and Quad

        # Left Face
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)  # Bottom Left Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 0.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)  # Bottom Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 1.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)  # Top Right Of The Texture and Quad
        self.gl_multi_tex_coord2f(self.gl_texture_0, 0.0, 1.0)
        self.gl_multi_tex_coord2f(self.gl_texture_1, 0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)  # Top Left Of The Texture and Quad

        gl.glEnd()  # Done Drawing The Cube

        self.rot = (self.rot + 0.2) % 360  # rotation

        # since this is double buffered, swap the buffers to display what just got drawn.
        glut.glutSwapBuffers()

    def key_pressed(self, *args):
        """The function called whenever a key is pressed.

        Note the use of Python tuples to pass in: (key, x, y)
        """
        # If escape is pressed, kill everything.
        if args[0] == self.ESCAPE:
            glut.glutDestroyWindow(self.window)

    def run(self):
        """Main entry point for the lesson."""
        glut.glutInit(sys.argv)

        # Select type of Display mode:
        #  Double buffer
        #  RGBA color
        # Alpha components supported
        # Depth buffer
        glut.glutInitDisplayMode(
            int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
        )

        # get a 640 x 480 window
        glut.glutInitWindowSize(640, 480)

        # the window starts at the upper left corner of the screen
        glut.glutInitWindowPosition(0, 0)

        # Create the window
        self.window = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )

        # Register the drawing function with glut
        glut.glutDisplayFunc(self.draw_scene)

        # Uncomment this line to get full screen.
        # glut.glutFullScreen()

        # When we are doing nothing, redraw the scene.
        glut.glutIdleFunc(self.draw_scene)

        # Register the function called when our window is resized.
        glut.glutReshapeFunc(self.resize_scene)

        # Register the function called when the keyboard is pressed.
        glut.glutKeyboardFunc(self.key_pressed)

        # Print message to console, and kick off the main to get it rolling.
        print("Hit ESC key to quit.")

        # Initialize our window.
        self.init_gl(640, 480)

        # Start Event Processing Engine
        glut.glutMainLoop()


def main():
    """Entry point for the lesson."""
    lesson = Lesson6Multi()
    lesson.run()


if __name__ == "__main__":
    main()
