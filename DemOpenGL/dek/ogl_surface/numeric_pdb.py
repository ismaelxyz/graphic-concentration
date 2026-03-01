import copy
import sys
from typing import Iterable, Optional

import numpy as np
from numpy import add, concatenate, cos, sin, subtract
from numpy.typing import NDArray

GUAAtoms = ["N2", "O6", "C6", "C5", "N7", "C8", "N9", "C4", "N3", "C2", "N1"]


def rotmat(phi, theta, psi, tx=0, ty=0, tz=0):

    s1 = sin(phi)
    s2 = sin(theta)
    s3 = sin(psi)
    c1 = cos(phi)
    c2 = cos(theta)
    c3 = cos(psi)

    newmat = np.array(
        [
            [c2 * c3, s2 * s1 * c3 - c1 * s3, s2 * c1 * c3 + s1 * s3, 0],
            [c2 * s3, s2 * s1 * s3 + c1 * c3, s2 * c1 * s3 - s1 * c3, 0],
            [-s2, c2 * s1, c2 * c1, 0],
            [tx, ty, tz, 1],
        ]
    )

    return newmat


def matrix_apply(crd, mat):
    o = [crd[0], crd[1], crd[2], 1.0]
    n = [0.0, 0.0, 0.0, 0.0]

    for i in range(4):
        for j in range(4):
            n[i] = n[i] + o[j] * mat[j][i]

    inv_w = 1.0 / n[3] if n[3] != 0.0 else 0.0
    return [n[0] * inv_w, n[1] * inv_w, n[2] * inv_w]


class PDBRecord:
    def __init__(
        self, xtype=None, anum=None, atom=None, residue=None, chain=None, rnum=None
    ):

        self.type = xtype
        self.anum = anum
        self.atom = atom
        self.residue = residue
        self.chain = chain
        self.rnum = rnum


class PDB:
    def __init__(
        self,
        filename: Optional[str] = None,
        crds: Optional[Iterable[tuple[float, float, float]]] = None,
        records: Optional[list[PDBRecord]] = None,
        connect: Optional[list[list[int]]] = None,
    ):
        self.records: list[PDBRecord] = records if records is not None else []
        crds_iter = crds if crds is not None else []
        self.crds: NDArray[np.floating] = np.array(list(crds_iter), dtype=float)
        if self.crds.size == 0:
            self.crds = np.empty((0, 3), dtype=float)

        self.connect = connect

        if filename is not None:
            sys.stderr.write(f"Reading in new PDB {filename}\n")
            self.read(filename)

    # NOTE: We read the anum here, which doesn't necessarily correspond to the
    # actual record number.

    def read(self, filename: str) -> None:
        sys.stderr.write(f"Opened '{filename}' for reading as PDB\n")

        records: list[PDBRecord] = []
        crds_list: list[tuple[float, float, float]] = []

        with open(filename, "r", encoding="utf-8", errors="replace") as pdbfile:
            for line in pdbfile:
                if line.startswith("ATOM") or line.startswith("HETATM"):
                    xtype = line[0:6].strip()
                    anum = int(line[6:11].strip() or 0)
                    atom = line[12:17].strip()
                    residue = line[17:20].strip()
                    chain = line[21:22].strip()
                    rnum = int(line[22:26].strip() or 0)
                    x = float(line[30:38].strip() or 0.0)
                    y = float(line[38:46].strip() or 0.0)
                    z = float(line[46:54].strip() or 0.0)

                    records.append(PDBRecord(xtype, anum, atom, residue, chain, rnum))
                    crds_list.append((x, y, z))

        self.records = records
        self.crds = np.array(crds_list, dtype=float)
        if self.crds.size == 0:
            self.crds = np.empty((0, 3), dtype=float)

    def write(self, filename: str) -> None:
        with open(filename, "w", encoding="utf-8") as pdbfd:
            for i, record in enumerate(self.records):
                pdbfd.write(
                    "%-6s%5d %-4s%c%-4s%c%4d%c   %8.3f%8.3f%8.3f\n"
                    % (
                        record.type,
                        record.anum,
                        record.atom,
                        " ",
                        record.residue,
                        " ",
                        record.rnum,
                        " ",
                        float(self.crds[i][0]),
                        float(self.crds[i][1]),
                        float(self.crds[i][2]),
                    )
                )

            pdbfd.write("TER\n")

            if self.connect is not None:
                for i in self.connect:
                    pdbfd.write("CONECT")
                    for j in i:
                        pdbfd.write("%5d" % (j + 1))
                    pdbfd.write("\n")

            pdbfd.write("END\n")

    def rotate(self, alpha, beta, gamma, tx, ty, tz) -> None:
        r = rotmat(alpha, beta, gamma, tx, ty, tz)
        self.rotate_matrix(r)

    def rotate_matrix(self, r) -> None:
        for i in range(len(self.crds)):
            self.crds[i] = matrix_apply(self.crds[i], r)

    def center(self) -> None:
        center = add.reduce(self.crds) / len(self.crds)
        self.crds = subtract(self.crds, center)

    def print_records(self) -> None:
        for i in self.records:
            print(i.type, i.anum, i.atom, i.residue, i.chain, i.rnum)

    def return_anum(self, atom, rnum):
        for i in range(len(self.records)):
            record = self.records[i]
            if record.atom == atom and record.rnum == rnum:
                return i
        sys.stderr.write('Unable to find atom "%s" in residue %s\n' % (atom, rnum))
        return None

    def crd_by_name(self, atom, res):
        x = self.return_anum(atom, res)
        if x is None:
            return None
        return self.crds[x]

    def fix_res_num(self) -> None:
        rnum = 1
        for i in range(len(self.records)):
            newrnum = rnum
            if (
                i < len(self.records) - 1
                and self.records[i + 1].rnum != self.records[i].rnum
            ):
                rnum = rnum + 1
            self.records[i].rnum = newrnum
            self.records[i].anum = i

    def copy(self):
        crds = np.array(self.crds)
        records = copy.copy(self.records)
        return PDB(None, crds, records)

    def append(self, struct2):
        records = self.records + copy.copy(struct2.records)
        crds = concatenate((self.crds, struct2.crds))

        return PDB(None, crds, records)
