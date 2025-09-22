# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from tkinter import *
from tkinter import ttk


class Gui:
    """The main CoinTracker GUI window"""

    def __init__(self):
        self.root = Tk()
        self.frame = ttk.Frame(self.root, padding=10)
        self.frame.grid()
        ttk.Label(self.frame, text="Hello World!").grid(column=0, row=0)
        ttk.Button(self.frame, text="Quit", command=self.root.destroy).grid(
            column=1, row=0
        )
        self.root.mainloop()
