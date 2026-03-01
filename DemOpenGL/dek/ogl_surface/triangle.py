#!/usr/bin/env python3
from pathlib import Path

import numpy as np
from OpenGL.GL import (
    GL_BACK,
    GL_BLEND,
    GL_COLOR_MATERIAL,
    GL_COMPILE,
    GL_CULL_FACE,
    GL_DEPTH_TEST,
    GL_DIFFUSE,
    GL_FRONT,
    GL_LIGHTING,
    GL_LINES,
    GL_TRIANGLES,
    GL_TRUE,
    glBegin,
    glCallList,
    glColor3f,
    glColorMaterial,
    glCullFace,
    glDepthMask,
    glDisable,
    glEnable,
    glEnd,
    glEndList,
    glGenLists,
    glNewList,
    glNormal3fv,
    glVertex3f,
    glVertex3fv,
)

from PIL import Image

try:
    from .gl_frame import GLFrame
    from . import numeric_pdb
except ImportError:  # pragma: no cover
    from gl_frame import GLFrame
    import numeric_pdb

HERE = Path(__file__).resolve().parent
DEFAULT_MAP = HERE / "test.ppm"


class Surface:

    def __init__(
        self,
        facefile: str = "1crn.face",
        vertfile: str = "1crn.vert",
        pdbfile: str = "1crn.pdb",
    ):
        self.facefile = str(
            (HERE / facefile) if not Path(facefile).is_absolute() else Path(facefile)
        )
        self.vertfile = str(
            (HERE / vertfile) if not Path(vertfile).is_absolute() else Path(vertfile)
        )
        self.pdbfile = str(
            (HERE / pdbfile) if not Path(pdbfile).is_absolute() else Path(pdbfile)
        )

        self.map = str(DEFAULT_MAP)

        alpha = 1
        self.colordict = {
            "C": [0.5, 0.5, 0.5, alpha],
            "O": [1, 0, 0, alpha],
            "N": [0, 0, 1, alpha],
            "S": [1, 1, 0, alpha],
            "P": [1, 0, 1, alpha],
            "H": [1, 1, 1, alpha],
            "U": [0, 0, 0, alpha],
        }

        self.mapdict = {
            "C": 0.5,
            "O": 0.9,
            "N": 0.1,
            "S": 0.5,
            "P": 0.5,
            "H": 0.5,
            "U": 0.5,
        }
        self.surface = None
        self.bond = None

        self.topol: list[tuple[int, int]] = []
        self.colorlist2: list[tuple[float, float, float]] = []
        self.crd = np.empty((0, 3), dtype=float)

        self.setup_window()

        self.read_pdb()
        self.make_image()
        self.read_surface()
        self.setup_surface()

        # self.OglFrame.ogl.tkRedraw()
        # out=tkinter.dooneevent(tkinter.DONT_WAIT)
        # while (out):
        # out=tkinter.dooneevent(tkinter.DONT_WAIT)
        # self.OglFrame.Photo()

        self.ogl_frame.mainloop()

    def setup_window(self):
        self.ogl_frame = GLFrame(
            None,
            redraw=self.display,
            depth=1,
            double=1,
        )
        self.ogl_frame.ogl.set_background(0, 0, 0)

    def display(self, event=None):
        glEnable(GL_DEPTH_TEST)
        glDepthMask(GL_TRUE)
        glEnable(GL_CULL_FACE)
        glCullFace(GL_BACK)

        if self.surface:
            glCallList(self.surfacelist)
        if self.bond:
            glCallList(self.bondlist)

    def setup_surface(self):
        self.surfacelist = glGenLists(1)
        glNewList(self.surfacelist, GL_COMPILE)

        glEnable(GL_LIGHTING)
        glEnable(GL_BLEND)

        glColorMaterial(GL_FRONT, GL_DIFFUSE)
        glEnable(GL_COLOR_MATERIAL)
        glBegin(mode=GL_TRIANGLES)
        for tri in self.faces:
            i1, i2, i3 = int(tri[0]) - 1, int(tri[1]) - 1, int(tri[2]) - 1
            vert1 = tuple(self.vert[i1])
            vert2 = tuple(self.vert[i2])
            vert3 = tuple(self.vert[i3])
            norm1 = tuple(self.norm[i1])
            norm2 = tuple(self.norm[i2])
            norm3 = tuple(self.norm[i3])

            color = self.colorlist[int(self.nearest[i1]) - 1]
            glColor3f(color[0], color[1], color[2])
            glNormal3fv(norm1)
            glVertex3fv(vert1)

            color = self.colorlist[int(self.nearest[i2]) - 1]
            glColor3f(color[0], color[1], color[2])
            glNormal3fv(norm2)
            glVertex3fv(vert2)

            color = self.colorlist[int(self.nearest[i3]) - 1]
            glColor3f(color[0], color[1], color[2])
            glNormal3fv(norm3)
            glVertex3fv(vert3)

        glEnd()
        glDisable(GL_LIGHTING)
        glDisable(GL_BLEND)
        glEndList()

        self.surface = 1

    def setup_bonds(self):
        self.bond = 1
        self.bondlist = glGenLists(1)
        glNewList(self.bondlist, GL_COMPILE)
        glDisable(GL_LIGHTING)
        glDisable(GL_BLEND)
        glBegin(mode=GL_LINES)

        for at1, at2 in self.topol:
            color = self.colorlist2[at1]
            glColor3f(*color)
            glVertex3f(
                float(self.crd[at1][0]),
                float(self.crd[at1][1]),
                float(self.crd[at1][2]),
            )
            color = self.colorlist2[at2]
            glColor3f(*color)
            glVertex3f(
                float(self.crd[at2][0]),
                float(self.crd[at2][1]),
                float(self.crd[at2][2]),
            )

        glEnd()
        glEnable(GL_BLEND)
        glEndList()

    def read_surface(self):
        with open(self.facefile, "r", encoding="utf-8", errors="replace") as f:
            l = f.readlines()

        data = l[2].split()
        numfaces = int(data[0])
        spheres = int(data[1])
        probe_r = float(data[2])
        density = float(data[3])
        print("Numfaces, spheres, probe_r, density")
        print(numfaces, spheres, probe_r, density)

        self.faces = np.zeros((numfaces, 3), dtype=int)
        for i in range(numfaces):
            data = l[i + 3].split()
            self.faces[i] = [int(x) for x in data[:3]]

        with open(self.vertfile, "r", encoding="utf-8", errors="replace") as f:
            l = f.readlines()

        data = l[2].split()
        vertices = int(data[0])
        spheres = int(data[1])
        probe_r = float(data[2])
        density = float(data[3])
        print("Vertices, spheres, probe_r, density")
        print(vertices, spheres, probe_r, density)

        self.vert = np.zeros((vertices, 3), dtype=float)
        self.norm = np.zeros((vertices, 3), dtype=float)
        self.nearest = np.zeros((vertices,), dtype=int)

        for i in range(vertices):
            data = l[i + 3].split()
            self.vert[i] = [float(x) for x in data[:3]]
            self.norm[i] = [float(x) for x in data[3:6]]
            self.nearest[i] = int(data[7])

        vcen = self.vert.mean(axis=0)
        self.ogl_frame.ogl.set_centerpoint(
            float(vcen[0]), float(vcen[1]), float(vcen[2])
        )

    def read_pdb(self):
        p = numeric_pdb.PDB(self.pdbfile)
        self.crd = p.crds

        atomlist = (x.atom for x in p.records)
        k = set(self.colordict.keys())
        self.colorlist = []
        self.maplist = []
        for atom in atomlist:
            if not atom:
                continue
            element = atom[0]
            if element in k:
                self.colorlist.append(self.colordict[element])
                self.maplist.append(self.mapdict[element])
            else:
                print("Unfound atom type:", atom)
                self.colorlist.append(self.colordict["U"])
                self.maplist.append(self.mapdict["U"])

    def make_image(self):
        im = Image.open(self.map)
        self.imageWidth = im.size[0]
        self.imageHeight = im.size[1]
        self.image = im.tobytes("raw", "RGBX", 0, -1)


if __name__ == "__main__":
    s = Surface()
