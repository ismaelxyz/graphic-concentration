#!/usr/bin/env python3

import sys
from pathlib import Path

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
from PIL import Image


class Lesson7:
    """OpenGL lesson 7 - Texture filters + lighting + input controls."""

    ESCAPE = b"\033"

    def __init__(self) -> None:
        self.window = 0

        self.light_enabled = False
        self.filter_index = 0

        self.xrot = 0.0
        self.yrot = 0.0
        self.xspeed = 0.0
        self.yspeed = 0.0
        self.z = -5.0

        self.light_ambient = (0.5, 0.5, 0.5, 1.0)
        self.light_diffuse = (1.0, 1.0, 1.0, 1.0)
        self.light_position = (0.0, 0.0, 2.0, 1.0)

        self.textures: list[int] = []

    def _texture_path(self) -> Path:
        return Path(__file__).with_name("crate.bmp")

    def load_textures(self) -> None:
        image_path = self._texture_path()
        image = Image.open(image_path)
        image = image.convert("RGB")

        width, height = image.size
        image_data = image.tobytes("raw", "RGB", 0, -1)

        self.textures = list(gl.glGenTextures(3))

        gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)

        # texture 1 (nearest)
        gl.glBindTexture(gl.GL_TEXTURE_2D, self.textures[0])
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_NEAREST)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_NEAREST)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            gl.GL_RGB,
            width,
            height,
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

        # texture 2 (linear)
        gl.glBindTexture(gl.GL_TEXTURE_2D, self.textures[1])
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR)
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            gl.GL_RGB,
            width,
            height,
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

        # texture 3 (mipmapped)
        gl.glBindTexture(gl.GL_TEXTURE_2D, self.textures[2])
        gl.glTexParameteri(gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MAG_FILTER, gl.GL_LINEAR)
        gl.glTexParameteri(
            gl.GL_TEXTURE_2D, gl.GL_TEXTURE_MIN_FILTER, gl.GL_LINEAR_MIPMAP_NEAREST
        )
        gl.glTexImage2D(
            gl.GL_TEXTURE_2D,
            0,
            gl.GL_RGB,
            width,
            height,
            0,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )
        # match the original C tutorial behavior
        glu.gluBuild2DMipmaps(
            gl.GL_TEXTURE_2D,
            gl.GL_RGB,
            width,
            height,
            gl.GL_RGB,
            gl.GL_UNSIGNED_BYTE,
            image_data,
        )

    def init_gl(self, width: int, height: int) -> None:
        self.load_textures()
        gl.glEnable(gl.GL_TEXTURE_2D)

        gl.glClearColor(0.0, 0.0, 0.0, 0.0)
        gl.glClearDepth(1.0)
        gl.glDepthFunc(gl.GL_LESS)
        gl.glEnable(gl.GL_DEPTH_TEST)
        gl.glShadeModel(gl.GL_SMOOTH)

        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

        # light number 1
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_AMBIENT, self.light_ambient)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_DIFFUSE, self.light_diffuse)
        gl.glLightfv(gl.GL_LIGHT1, gl.GL_POSITION, self.light_position)
        gl.glEnable(gl.GL_LIGHT1)

        self._apply_lighting_state()

    def resize_scene(self, width: int, height: int) -> None:
        if height == 0:
            height = 1

        gl.glViewport(0, 0, width, height)
        gl.glMatrixMode(gl.GL_PROJECTION)
        gl.glLoadIdentity()
        glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
        gl.glMatrixMode(gl.GL_MODELVIEW)

    def _apply_lighting_state(self) -> None:
        if self.light_enabled:
            gl.glEnable(gl.GL_LIGHTING)
        else:
            gl.glDisable(gl.GL_LIGHTING)

    def draw_scene(self) -> None:
        gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))
        gl.glLoadIdentity()

        gl.glTranslatef(0.0, 0.0, float(self.z))
        gl.glRotatef(float(self.xrot), 1.0, 0.0, 0.0)
        gl.glRotatef(float(self.yrot), 0.0, 1.0, 0.0)

        if self.textures:
            gl.glBindTexture(gl.GL_TEXTURE_2D, self.textures[self.filter_index])

        gl.glBegin(gl.GL_QUADS)

        # Front Face
        gl.glNormal3f(0.0, 0.0, 1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)

        # Back Face
        gl.glNormal3f(0.0, 0.0, -1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)

        # Top Face
        gl.glNormal3f(0.0, 1.0, 0.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)

        # Bottom Face
        gl.glNormal3f(0.0, -1.0, 0.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)

        # Right Face
        gl.glNormal3f(1.0, 0.0, 0.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(1.0, 1.0, -1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(1.0, -1.0, 1.0)

        # Left Face
        gl.glNormal3f(-1.0, 0.0, 0.0)
        gl.glTexCoord2f(0.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, -1.0)
        gl.glTexCoord2f(1.0, 0.0)
        gl.glVertex3f(-1.0, -1.0, 1.0)
        gl.glTexCoord2f(1.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, 1.0)
        gl.glTexCoord2f(0.0, 1.0)
        gl.glVertex3f(-1.0, 1.0, -1.0)

        gl.glEnd()

        self.xrot += self.xspeed
        self.yrot += self.yspeed

        glut.glutSwapBuffers()

    def key_pressed(self, key: bytes, x: int, y: int) -> None:  # noqa: ARG002
        if key == self.ESCAPE:
            glut.glutDestroyWindow(self.window)
            return

        if key in (b"l", b"L"):
            self.light_enabled = not self.light_enabled
            self._apply_lighting_state()
            return

        if key in (b"f", b"F"):
            self.filter_index = (self.filter_index + 1) % 3
            return

    def special_key_pressed(self, key: int, x: int, y: int) -> None:  # noqa: ARG002
        if key == glut.GLUT_KEY_PAGE_UP:
            self.z -= 0.02
            return

        if key == glut.GLUT_KEY_PAGE_DOWN:
            self.z += 0.02
            return

        if key == glut.GLUT_KEY_UP:
            self.xspeed -= 0.01
            return

        if key == glut.GLUT_KEY_DOWN:
            self.xspeed += 0.01
            return

        if key == glut.GLUT_KEY_LEFT:
            self.yspeed -= 0.01
            return

        if key == glut.GLUT_KEY_RIGHT:
            self.yspeed += 0.01
            return

    def run(self) -> None:
        glut.glutInit(sys.argv)
        glut.glutInitDisplayMode(
            int(glut.GLUT_RGBA)
            | int(glut.GLUT_DOUBLE)
            | int(glut.GLUT_ALPHA)
            | int(glut.GLUT_DEPTH)
        )
        glut.glutInitWindowSize(640, 480)
        glut.glutInitWindowPosition(0, 0)

        self.window = glut.glutCreateWindow(
            "Jeff Molofee's GL Code Tutorial ... NeHe '99"
        )

        glut.glutDisplayFunc(self.draw_scene)
        glut.glutIdleFunc(self.draw_scene)
        glut.glutReshapeFunc(self.resize_scene)
        glut.glutKeyboardFunc(self.key_pressed)
        glut.glutSpecialFunc(self.special_key_pressed)

        glut.glutFullScreen()

        self.init_gl(640, 480)
        glut.glutMainLoop()


def main() -> None:
    lesson = Lesson7()
    lesson.run()


if __name__ == "__main__":
    main()
