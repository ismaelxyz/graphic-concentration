#!/usr/bin/env python3
# -*- coding: utf-8 -*-


from numpy import arange, ravel, zeros, newaxis, array, where, greater, resize
from tkinter import Tk, Label, Frame
from PIL import Image, ImageTk
import sys


class Scene(Frame):

    def __init__(self, master, width=603, height=456):
        super().__init__(master, width=width, height=height)

        self.im = self.mandel = self.background = None

    def draw(self, LowX, HighX, LowY, HighY, maxiter=30):
        xx = arange(LowX, HighX, (HighX-LowX) / self['width'] * 2)
        yy = arange(HighY, LowY, (LowY-HighY) / self['height'] * 2) * 1j
        c = ravel(xx + yy[:, newaxis])
        z = zeros(c.shape, complex)
        output = resize(array(0,), c.shape)

        for iter in range(maxiter):
            print('iter:', iter)
            z = z * z + c
            finished = greater(abs(z), 2.0)
            c = where(finished, 0+0j, c)
            z = where(finished, 0+0j, z)
            output = where(finished, iter, output)

        # scale output a bit to make it brighter
        # output * output * 1000
        output = (output + (256 * output) + (256**2) * output) * 8
        self.mandel = output.tobytes()
        print('Size model:', len(self.mandel))

    def createImage(self):
        self.im = Image.new('RGB', (self['width']//2, self['height']//2))
        self.draw(-2.1, 0.7, -1.2, 1.2)

        # Size of image in bytes
        print('Image bytes:', len(self.im.tobytes('raw', 'RGBX', 0, -1)))
        self.im.frombytes(self.mandel, 'raw', 'RGBX', 0, -1)

    def createLabel(self):
        self.image = ImageTk.PhotoImage(self.im)
        self.background = Label(self.master, image=self.image)
        self.background.pack()


def main():
    window = Tk()
    window.title('Mandel Image')
    scene = Scene(window)
    scene.createImage()
    scene.createLabel()
    window.mainloop()


if __name__ == '__main__':
    main()
