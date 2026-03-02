#!/usr/bin/env python3

import os
import random
import sys
from dataclasses import dataclass
from pathlib import Path

import OpenGL.GL as gl
import OpenGL.GLU as glu
import OpenGL.GLUT as glut
import OpenGL.raw.GLUT as raw_glut
from PIL import Image


STAR_NUM = 50


@dataclass(slots=True)
class Star:
	r: int
	g: int
	b: int
	dist: float
	angle: float


class Lesson9:
	"""OpenGL lesson 9 - Moving bitmaps in 3D space (starfield)."""

	ESCAPE = b"\033"

	def __init__(self) -> None:
		self.window = 0

		self.twinkle = False

		self.zoom = -15.0
		self.tilt = 90.0
		self.spin = 0.0

		self.texture_id: int | None = None
		self.stars: list[Star] = []

		self._should_exit = False
		self._c_callbacks: list[object] = []

	def _texture_path(self) -> Path:
		return Path(__file__).with_name("Star.bmp")

	def load_textures(self) -> None:
		image_path = self._texture_path()
		image = Image.open(image_path)
		image = image.convert("RGB")

		width, height = image.size
		image_data = image.tobytes("raw", "RGB", 0, -1)

		texture_id = int(gl.glGenTextures(1))
		self.texture_id = texture_id

		gl.glBindTexture(gl.GL_TEXTURE_2D, texture_id)
		gl.glPixelStorei(gl.GL_UNPACK_ALIGNMENT, 1)
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

	def _init_stars(self) -> None:
		self.stars.clear()
		for index in range(STAR_NUM):
			dist = (index * 1.0 / STAR_NUM) * 5.0
			self.stars.append(
				Star(
					r=random.randrange(256),
					g=random.randrange(256),
					b=random.randrange(256),
					dist=dist,
					angle=0.0,
				)
			)

	def init_gl(self, width: int, height: int) -> None:
		self.load_textures()
		gl.glEnable(gl.GL_TEXTURE_2D)

		gl.glClearColor(0.0, 0.0, 0.0, 0.0)
		gl.glClearDepth(1.0)
		gl.glShadeModel(gl.GL_SMOOTH)

		gl.glMatrixMode(gl.GL_PROJECTION)
		gl.glLoadIdentity()
		glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
		gl.glMatrixMode(gl.GL_MODELVIEW)

		gl.glBlendFunc(gl.GL_SRC_ALPHA, gl.GL_ONE)
		gl.glEnable(gl.GL_BLEND)

		self._init_stars()

	def resize_scene(self, width: int, height: int) -> None:
		if height == 0:
			height = 1

		gl.glViewport(0, 0, width, height)
		gl.glMatrixMode(gl.GL_PROJECTION)
		gl.glLoadIdentity()
		glu.gluPerspective(45.0, float(width) / float(height), 0.1, 100.0)
		gl.glMatrixMode(gl.GL_MODELVIEW)

	def draw_scene(self) -> None:
		if self._should_exit:
			try:
				if self.window:
					raw_glut.glutDestroyWindow(int(self.window))
			finally:
				os._exit(0)

		gl.glClear(int(gl.GL_COLOR_BUFFER_BIT) | int(gl.GL_DEPTH_BUFFER_BIT))

		if self.texture_id is not None:
			gl.glBindTexture(gl.GL_TEXTURE_2D, int(self.texture_id))

		for index, star in enumerate(self.stars):
			gl.glLoadIdentity()
			gl.glTranslatef(0.0, 0.0, float(self.zoom))
			gl.glRotatef(float(self.tilt), 1.0, 0.0, 0.0)

			gl.glRotatef(float(star.angle), 0.0, 1.0, 0.0)
			gl.glTranslatef(float(star.dist), 0.0, 0.0)

			gl.glRotatef(float(-star.angle), 0.0, 1.0, 0.0)
			gl.glRotatef(float(-self.tilt), 1.0, 0.0, 0.0)

			if self.twinkle:
				twinkle_star = self.stars[(STAR_NUM - index - 1) % STAR_NUM]
				gl.glColor4ub(
					int(twinkle_star.r),
					int(twinkle_star.g),
					int(twinkle_star.b),
					255,
				)
				gl.glBegin(gl.GL_QUADS)
				gl.glTexCoord2f(0.0, 0.0)
				gl.glVertex3f(-1.0, -1.0, 0.0)
				gl.glTexCoord2f(1.0, 0.0)
				gl.glVertex3f(1.0, -1.0, 0.0)
				gl.glTexCoord2f(1.0, 1.0)
				gl.glVertex3f(1.0, 1.0, 0.0)
				gl.glTexCoord2f(0.0, 1.0)
				gl.glVertex3f(-1.0, 1.0, 0.0)
				gl.glEnd()

			gl.glRotatef(float(self.spin), 0.0, 0.0, 1.0)
			gl.glColor4ub(int(star.r), int(star.g), int(star.b), 255)
			gl.glBegin(gl.GL_QUADS)
			gl.glTexCoord2f(0.0, 0.0)
			gl.glVertex3f(-1.0, -1.0, 0.0)
			gl.glTexCoord2f(1.0, 0.0)
			gl.glVertex3f(1.0, -1.0, 0.0)
			gl.glTexCoord2f(1.0, 1.0)
			gl.glVertex3f(1.0, 1.0, 0.0)
			gl.glTexCoord2f(0.0, 1.0)
			gl.glVertex3f(-1.0, 1.0, 0.0)
			gl.glEnd()

			self.spin += 0.01
			star.angle += (index * 1.0 / STAR_NUM) * 1.0
			star.dist -= 0.01
			if star.dist < 0.0:
				star.dist += 5.0
				star.r = random.randrange(256)
				star.g = random.randrange(256)
				star.b = random.randrange(256)

		glut.glutSwapBuffers()

	def key_pressed(self, key: bytes, x: int, y: int) -> None:  # noqa: ARG002
		if key == self.ESCAPE:
			self._should_exit = True
			return

		if key in (b"t", b"T"):
			self.twinkle = not self.twinkle
			return

	def special_key_pressed(self, key: int, x: int, y: int) -> None:  # noqa: ARG002
		if key == glut.GLUT_KEY_PAGE_UP:
			self.zoom -= 0.2
			return

		if key == glut.GLUT_KEY_PAGE_DOWN:
			self.zoom += 0.2
			return

		if key == glut.GLUT_KEY_UP:
			self.tilt -= 0.5
			return

		if key == glut.GLUT_KEY_DOWN:
			self.tilt += 0.5
			return

	def _keyboard_callback(self, key: int, x: int, y: int) -> None:
		try:
			self.key_pressed(bytes((int(key) & 0xFF,)), int(x), int(y))
		except Exception as exc:
			print(f"keyboard callback error: {exc}", file=sys.stderr)

	def _special_callback(self, key: int, x: int, y: int) -> None:
		try:
			self.special_key_pressed(int(key), int(x), int(y))
		except Exception as exc:
			print(f"special-key callback error: {exc}", file=sys.stderr)

	def _register_callbacks_raw(self) -> None:
		self._c_callbacks.clear()

		display_cb = raw_glut.CALLBACK_FUNCTION_TYPE(None)(self.draw_scene)
		idle_cb = raw_glut.CALLBACK_FUNCTION_TYPE(None)(self.draw_scene)
		reshape_cb = raw_glut.CALLBACK_FUNCTION_TYPE(None, raw_glut.c_int, raw_glut.c_int)(
			self.resize_scene
		)
		keyboard_cb = raw_glut.CALLBACK_FUNCTION_TYPE(
			None, raw_glut.c_ubyte, raw_glut.c_int, raw_glut.c_int
		)(self._keyboard_callback)
		special_cb = raw_glut.CALLBACK_FUNCTION_TYPE(None, raw_glut.c_int, raw_glut.c_int, raw_glut.c_int)(
			self._special_callback
		)

		self._c_callbacks.extend(
			[display_cb, idle_cb, reshape_cb, keyboard_cb, special_cb]
		)

		raw_glut.glutDisplayFunc(display_cb)
		raw_glut.glutIdleFunc(idle_cb)
		raw_glut.glutReshapeFunc(reshape_cb)
		raw_glut.glutKeyboardFunc(keyboard_cb)
		raw_glut.glutSpecialFunc(special_cb)

	def run(self) -> None:
		if not os.environ.get("DISPLAY") and not os.environ.get("WAYLAND_DISPLAY"):
			raise RuntimeError(
				"No display detected (DISPLAY/WAYLAND_DISPLAY not set). "
				"Run inside a desktop session or set up X11 forwarding."
			)

		try:
			glut.glutInit(sys.argv)
		except TypeError:
			glut.glutInit()

		glut.glutInitDisplayMode(
			int(glut.GLUT_RGBA) | int(glut.GLUT_DOUBLE) | int(glut.GLUT_DEPTH)
		)
		glut.glutInitWindowSize(640, 480)
		glut.glutInitWindowPosition(0, 0)

		self.window = glut.glutCreateWindow(
			"Jeff Molofee's GL Code Tutorial ... NeHe '99"
		)
		if not self.window:
			raise RuntimeError(
				"glutCreateWindow() failed (no valid window/context). "
				"Check your OpenGL/GLUT setup and that a display server is available."
			)

		try:
			glut.glutSetWindow(self.window)
		except Exception:
			pass

		self._register_callbacks_raw()
		glut.glutFullScreen()

		self.init_gl(640, 480)
		glut.glutMainLoop()


def main() -> None:
	lesson = Lesson9()
	lesson.run()


if __name__ == "__main__":
	main()


